# Play set_entity_data Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_set_entity_data_clientbound_framed_dispatch` |
| Packet | `0x63` / `minecraft:set_entity_data` |
| Fixture | `ClientboundSetEntityDataPacket(12345, List.of())` |
| Body | `b960ff` |
| Scope | Packet framing and dispatch/decode for one entity id plus empty metadata list |

## Evidence

Official bytecode for `ClientboundSetEntityDataPacket` shows the body writes
the entity id as VarInt, then packs each `SynchedEntityData.DataValue`, then
writes EOF marker byte `255`. The selected fixture uses `List.of()`, so the
official answer writes only the EOF marker after the entity id and never enters
metadata serializer/value encoding.

## Stop Boundary

This proof does not establish entity existence, metadata serializers, metadata
values, initialized entity state, or client-load completion. Stevenarella
accepts only the empty metadata EOF-marker fixture and rejects non-empty
metadata markers before serializer decoding.
