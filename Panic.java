import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.function.Function;

class Panic {
    public static void main(String[] args) throws NotFoundException {
        run();
        blah();
        System.out.println("Still alive.");
    }

    static void run() {
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
            var codes = stringToCodes(text);
            Collections.reverse(codes);
            return codesToString(codes);
        } catch (NotFoundException exception) {
            throw new RuntimeException(exception);
        }
    }

    static List<String> texts = List.of("tar", "flow");
    // static List<String> texts = List.of("tar", "flow", "🐻‍❄️❤️🦭");

    static String retrieveText(int id) throws NotFoundException {
        if (id <= 0 || id > texts.size()) {
            var message = String.format("404 - Not found: %s", id);
            throw new NotFoundException(message);
        }
        return texts.get(id - 1);
    }

    static String codesToString(List<Integer> codes) {
        var array = codes.stream().mapToInt(Integer::intValue).toArray();
        // Throws on large ints.
        return new String(array, 0, array.length);
    }

    static List<Integer> stringToCodes(String text) {
        return text.codePoints().boxed().collect(
                Collectors.toCollection(ArrayList::new));
    }

    static String blech(String thing) {
        return thing + thing;
    }

    public static void blah() throws NotFoundException {
        // Fn<Integer, String, NotFoundException> parse = Panic::retrieveText;
        // Fn<String, String, NotFoundException> times2 = Panic::blech;
        var composed = Fn.compose(Panic::blech, Panic::retrieveText);
        System.out.println(composed.apply(1));
    }
}

class NotFoundException extends Exception {
    public NotFoundException(String message) {
        super(message);
    }
}

public interface Fn<A, B, E extends Throwable> {
    B apply(A a) throws E;

    static <T, U, R, E extends Throwable> Fn<T, R, E> compose(Fn<U, R, E> f, Fn<T, U, E> g) {
        return x -> f.apply(g.apply(x));
    }
}
