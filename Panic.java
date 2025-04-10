import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.IntStream;

class Panic {
    public static void main(String[] args) {
        run();
        System.out.println("Still alive.");
    }

    static void run() {
        // codesToString(List.of(0x110000));
        try {
            IntStream.range(1, 3)
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
            reverseInPlace(codes);
            return codesToString(codes);
        } catch (NotFoundException exception) {
            throw new RuntimeException(exception);
        }
    }

    static List<String> texts = List.of("tar", "flow");

    static String retrieveText(int id) throws NotFoundException {
        if (id <= 0 || id > texts.size()) {
            var message = String.format("404 - Not found: %s", id);
            throw new NotFoundException(message);
        }
        return texts.get(id - 1);
    }

    static <T> void reverseInPlace(List<T> values) {
        var size = values.size();
        for (var index = 0; index < size / 2; index += 1) {
            Collections.swap(values, index, size - index - 1);
        }
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
