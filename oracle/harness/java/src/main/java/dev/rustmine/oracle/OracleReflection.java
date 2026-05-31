package dev.rustmine.oracle;

import java.lang.reflect.Field;
import java.util.List;


public final class OracleReflection {
    private OracleReflection() {
    }

    public static byte privateByte(Object target, String fieldName) {
        try {
            Field field = findField(target.getClass(), fieldName);
            return field.getByte(target);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read private byte field " + fieldName + " from " + target.getClass().getName(),
                err
            );
        }
    }

    public static int privateInt(Object target, String fieldName) {
        try {
            Field field = findField(target.getClass(), fieldName);
            return field.getInt(target);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read private int field " + fieldName + " from " + target.getClass().getName(),
                err
            );
        }
    }

    public static int privateListSize(Object target, String fieldName) {
        try {
            Field field = target.getClass().getDeclaredField(fieldName);
            field.setAccessible(true);
            Object value = field.get(target);
            if (!(value instanceof List<?> list)) {
                throw new IllegalStateException(
                    "private field " + fieldName + " is not a List on " + target.getClass().getName()
                );
            }
            return list.size();
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read private List field " + fieldName + " from " + target.getClass().getName(),
                err
            );
        }
    }

    public static byte[] privateByteArray(Object object, String fieldName) {
        try {
            Field field = object.getClass().getDeclaredField(fieldName);
            field.setAccessible(true);
            return (byte[]) field.get(object);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read official field " + fieldName + " from " + object.getClass().getName(),
                err
            );
        }
    }

    private static Field findField(Class<?> start, String fieldName) throws NoSuchFieldException {
        Class<?> current = start;
        while (current != null) {
            try {
                Field field = current.getDeclaredField(fieldName);
                field.setAccessible(true);
                return field;
            } catch (NoSuchFieldException ignored) {
                current = current.getSuperclass();
            }
        }
        throw new NoSuchFieldException(fieldName);
    }
}
