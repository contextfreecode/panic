using System.Text;

class Panic
{
    static void Main()
    {
        Run();
        Console.WriteLine("Still alive.");
    }

    static void Run()
    {
        try
        {
            foreach (var text in Enumerable.Range(1, 3).Select(ProcessText))
            {
                Console.WriteLine(text);
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"{ex.GetType()}: {ex.Message}");
        }
    }

    static string ProcessText(int id)
    {
        var text = RetrieveText(id);
        var runes = StringToRunes(text);
        ReverseInPlace(runes);
        return RunesToString(runes);
    }

    static readonly List<string> Texts = new() { "tar", "flow" };

    /// <exception cref="NotFoundException"></exception>
    static string RetrieveText(int id)
    {
        if (id <= 0 || id > Texts.Count)
        {
            throw new NotFoundException($"404 - Not found: {id}");
        }
        return Texts[id - 1];
    }

    static void ReverseInPlace<T>(List<T> values)
    {
        for (int index = 0; index < values.Count / 2; index++)
        {
            var reverseIndex = index + 1;
            (values[index], values[^reverseIndex]) =
                (values[^reverseIndex], values[index]);
        }
    }

    static string RunesToString(List<Rune> runes) => string.Concat(runes);

    static List<Rune> StringToRunes(string text) =>
        text.EnumerateRunes().ToList();
}

class NotFoundException : Exception
{
    public NotFoundException(string message) : base(message) { }
}
