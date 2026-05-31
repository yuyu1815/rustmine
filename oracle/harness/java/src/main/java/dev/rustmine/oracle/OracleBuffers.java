package dev.rustmine.oracle;

import java.util.HexFormat;
import net.minecraft.network.FriendlyByteBuf;


public final class OracleBuffers {
    private OracleBuffers() {
    }

    public static byte[] readableBytes(FriendlyByteBuf buffer) {
        byte[] bytes = new byte[buffer.readableBytes()];
        buffer.getBytes(buffer.readerIndex(), bytes);
        return bytes;
    }

    public static byte[] bytesAfterVarIntPrefix(byte[] framed) {
        int offset = 0;
        for (; offset < framed.length && offset < 5; offset += 1) {
            if ((framed[offset] & 0x80) == 0) {
                offset += 1;
                break;
            }
        }
        if (offset == 0 || offset > framed.length || offset > 5) {
            throw new IllegalStateException("missing complete VarInt packet id prefix");
        }
        byte[] body = new byte[framed.length - offset];
        System.arraycopy(framed, offset, body, 0, body.length);
        return body;
    }

    public static byte[] bytesAfterVarIntAndIdentifierPrefix(byte[] body) {
        int offset = varIntPrefixLength(body, 0);
        int identifierLengthPrefix = varIntPrefixLength(body, offset);
        int identifierLength = readVarInt(body, offset);
        offset += identifierLengthPrefix + identifierLength;
        if (offset > body.length) {
            throw new IllegalStateException("identifier length extends past body");
        }
        byte[] payloadBody = new byte[body.length - offset];
        System.arraycopy(body, offset, payloadBody, 0, payloadBody.length);
        return payloadBody;
    }

    public static byte[] hexToBytes(String value) {
        if (value.isEmpty()) {
            return new byte[0];
        }
        return HexFormat.of().parseHex(value);
    }

    private static int varIntPrefixLength(byte[] bytes, int offset) {
        for (int i = offset; i < bytes.length && i < offset + 5; i += 1) {
            if ((bytes[i] & 0x80) == 0) {
                return i - offset + 1;
            }
        }
        throw new IllegalStateException("missing complete VarInt prefix");
    }

    private static int readVarInt(byte[] bytes, int offset) {
        int value = 0;
        int shift = 0;
        for (int i = offset; i < bytes.length && i < offset + 5; i += 1) {
            int current = bytes[i] & 0xFF;
            value |= (current & 0x7F) << shift;
            if ((current & 0x80) == 0) {
                return value;
            }
            shift += 7;
        }
        throw new IllegalStateException("missing complete VarInt value");
    }
}
