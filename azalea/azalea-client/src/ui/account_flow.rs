//! Account selection and login flow state.
//!
//! This module follows Stevenarella's launcher storyboard without copying its
//! rendering stack. Presentation code can render these states, while Azalea's
//! existing account types remain responsible for authentication.

use std::{fs, io, path::Path};

use rand::random;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::account::Account;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredLauncherAccount {
    pub display_name: String,
    pub login: LauncherLogin,
}

impl StoredLauncherAccount {
    pub fn offline(username: impl Into<String>) -> Self {
        let username = username.into();
        Self {
            display_name: username.clone(),
            login: LauncherLogin::Offline { username },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LauncherLogin {
    Offline {
        username: String,
    },
    #[cfg(feature = "online-mode")]
    Microsoft {
        cache_key: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountFlowScreen {
    AccountSelection,
    LoginForm { error: Option<String> },
    LoggingIn { display_name: String },
}

#[derive(Clone, Debug)]
pub struct AccountFlow {
    accounts: Vec<StoredLauncherAccount>,
    active_account: Option<StoredLauncherAccount>,
    screen: AccountFlowScreen,
}

impl AccountFlow {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            accounts,
            active_account: None,
            screen: AccountFlowScreen::AccountSelection,
        }
    }

    pub fn accounts(&self) -> &[StoredLauncherAccount] {
        &self.accounts
    }

    pub fn active_account(&self) -> Option<&StoredLauncherAccount> {
        self.active_account.as_ref()
    }

    pub fn screen(&self) -> &AccountFlowScreen {
        &self.screen
    }

    pub fn show_login_form(&mut self) {
        self.screen = AccountFlowScreen::LoginForm { error: None };
    }

    pub fn cancel_login(&mut self) {
        self.screen = AccountFlowScreen::AccountSelection;
    }

    pub fn start_offline_login(&mut self, username: impl Into<String>) -> LauncherLoginRequest {
        let username = default_username(username.into());
        let display_name = username.clone();
        self.screen = AccountFlowScreen::LoggingIn {
            display_name: display_name.clone(),
        };
        LauncherLoginRequest {
            display_name,
            login: LauncherLogin::Offline { username },
        }
    }

    #[cfg(feature = "online-mode")]
    pub fn start_microsoft_login(&mut self, cache_key: impl Into<String>) -> LauncherLoginRequest {
        let cache_key = cache_key.into();
        self.screen = AccountFlowScreen::LoggingIn {
            display_name: cache_key.clone(),
        };
        LauncherLoginRequest {
            display_name: cache_key.clone(),
            login: LauncherLogin::Microsoft { cache_key },
        }
    }

    pub fn select_account(&mut self, index: usize) -> Option<LauncherLoginRequest> {
        let account = self.accounts.get(index)?;
        self.screen = AccountFlowScreen::LoggingIn {
            display_name: account.display_name.clone(),
        };
        Some(LauncherLoginRequest {
            display_name: account.display_name.clone(),
            login: account.login.clone(),
        })
    }

    pub fn finish_login_success(&mut self, request: LauncherLoginRequest, account: &Account) {
        let stored_account = request.into_stored_account(account);
        self.active_account = Some(stored_account.clone());
        if !self
            .accounts
            .iter()
            .any(|entry| entry.login == stored_account.login)
        {
            self.accounts.push(stored_account);
        }
        self.screen = AccountFlowScreen::AccountSelection;
    }

    pub fn finish_login_error(&mut self, message: impl Into<String>) {
        self.screen = AccountFlowScreen::LoginForm {
            error: Some(message.into()),
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LauncherLoginRequest {
    pub display_name: String,
    pub login: LauncherLogin,
}

impl LauncherLoginRequest {
    pub async fn authenticate(&self) -> Result<Account, LauncherLoginError> {
        match &self.login {
            LauncherLogin::Offline { username } => Ok(Account::offline(username)),
            #[cfg(feature = "online-mode")]
            LauncherLogin::Microsoft { cache_key } => Account::microsoft(cache_key)
                .await
                .map_err(LauncherLoginError::Microsoft),
        }
    }

    fn into_stored_account(self, account: &Account) -> StoredLauncherAccount {
        StoredLauncherAccount {
            display_name: account.username().to_owned(),
            login: self.login,
        }
    }
}

#[derive(Debug, Error)]
pub enum LauncherLoginError {
    #[cfg(feature = "online-mode")]
    #[error("{0}")]
    Microsoft(azalea_auth::AuthError),
}

#[derive(Debug, Error)]
pub enum LauncherAccountStoreError {
    #[error("failed to read launcher account store: {0}")]
    Read(#[from] io::Error),
    #[error("failed to parse launcher account store: {0}")]
    Parse(#[from] serde_json::Error),
}

pub fn load_accounts(
    path: impl AsRef<Path>,
) -> Result<Vec<StoredLauncherAccount>, LauncherAccountStoreError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn save_accounts(
    path: impl AsRef<Path>,
    accounts: &[StoredLauncherAccount],
) -> Result<(), io::Error> {
    let accounts = accounts
        .iter()
        .filter(|account| account.login.is_persistent())
        .collect::<Vec<_>>();
    let json = serde_json::to_string_pretty(&accounts)
        .expect("serializing launcher accounts should not fail");
    fs::write(path, json)
}

impl LauncherLogin {
    fn is_persistent(&self) -> bool {
        match self {
            LauncherLogin::Offline { .. } => true,
            #[cfg(feature = "online-mode")]
            LauncherLogin::Microsoft { .. } => false,
        }
    }
}

fn default_username(username: String) -> String {
    if username.is_empty() {
        format!("Player{}", random::<u8>())
    } else {
        username
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_offline_account_flow_returns_to_account_selection() {
        let mut flow = AccountFlow::new(Vec::new());

        flow.show_login_form();
        let request = flow.start_offline_login("Steve");
        assert_eq!(
            flow.screen(),
            &AccountFlowScreen::LoggingIn {
                display_name: "Steve".to_owned()
            }
        );

        let account = Account::offline("Steve");
        flow.finish_login_success(request, &account);

        assert_eq!(flow.screen(), &AccountFlowScreen::AccountSelection);
        assert_eq!(flow.active_account().unwrap().display_name, "Steve");
        assert_eq!(flow.accounts(), &[StoredLauncherAccount::offline("Steve")]);
    }

    #[test]
    fn selecting_existing_account_sets_logging_in_state() {
        let mut flow = AccountFlow::new(vec![StoredLauncherAccount::offline("Alex")]);

        let request = flow.select_account(0).unwrap();

        assert_eq!(
            request,
            LauncherLoginRequest {
                display_name: "Alex".to_owned(),
                login: LauncherLogin::Offline {
                    username: "Alex".to_owned()
                }
            }
        );
        assert_eq!(
            flow.screen(),
            &AccountFlowScreen::LoggingIn {
                display_name: "Alex".to_owned()
            }
        );
    }

    #[test]
    fn login_error_returns_to_form_with_message() {
        let mut flow = AccountFlow::new(Vec::new());

        flow.show_login_form();
        flow.finish_login_error("No account");

        assert_eq!(
            flow.screen(),
            &AccountFlowScreen::LoginForm {
                error: Some("No account".to_owned())
            }
        );
    }

    #[test]
    fn persistence_uses_stevenarella_policy_for_microsoft_accounts() {
        let temporary_file =
            std::env::temp_dir().join(format!("azalea-launcher-accounts-{}.json", random::<u64>()));
        let accounts = vec![
            StoredLauncherAccount::offline("Alex"),
            #[cfg(feature = "online-mode")]
            StoredLauncherAccount {
                display_name: "email@example.com".to_owned(),
                login: LauncherLogin::Microsoft {
                    cache_key: "email@example.com".to_owned(),
                },
            },
        ];

        save_accounts(&temporary_file, &accounts).unwrap();
        let loaded_accounts = load_accounts(&temporary_file).unwrap();
        let _ = fs::remove_file(&temporary_file);

        assert_eq!(
            loaded_accounts,
            vec![StoredLauncherAccount::offline("Alex")]
        );
    }
}
