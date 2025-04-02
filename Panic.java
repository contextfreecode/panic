import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

class Panic {
    public static void main(String[] args) {
        // codesToString(List.of(0x110000));
        try {
            IntStream.range(1, 4)
                .mapToObj(id -> processText(id))
                .forEach(text -> {
                    System.out.println(text);
                });
        } catch (Exception exception) {
            // exception.printStackTrace();
            System.err.println(exception);
        }
    }

    static String processText(int id) {
        try {
            var text = retrieveText(id);
            var codes = new ArrayList<>(stringToCodes(text));
            rotateBack(codes);
            return codesToString(codes);
        } catch (NotFoundException exception) {
            throw new RuntimeException(exception);
        }
    }

    static List<String> texts = List.of("smile", "tears");

    static String retrieveText(int id) throws NotFoundException {
        if (id <= 0 || id > texts.size()) {
            var message = String.format("404 - Not found: %s", id);
            throw new NotFoundException(message);
        }
        return texts.get(id - 1);
    }

    static <T> void rotateBack(List<T> vals) {
        var size = vals.size();
        // if (size == 0) {
        //     return;
        // }
        var first = vals.get(0);
        IntStream.range(1, size).forEach(index -> {
            vals.set(index - 1, vals.get(index));
        });
        vals.set(size - 1, first);
    }

    static String codesToString(List<Integer> codes) {
        var array = codes.stream().mapToInt(Integer::intValue).toArray();
        // Throws on large ints.
        return new String(array, 0, array.length);
    }

    static List<Integer> stringToCodes(String text) {
        return text.codePoints().boxed().toList();
    }
}

class NotFoundException extends Exception {
    public NotFoundException(String message) {
        super(message);
    }
}

@FunctionalInterface
interface ThrowingSupplier<T, E extends Exception> {
    T get() throws E;
}
