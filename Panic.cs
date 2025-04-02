using System.Text;

class Panic
{
    static void Main()
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
        RotateBack(runes);
        return RunesToString(runes);
    }

    static readonly List<string> Texts = new() { "smile", "tears" };

    /// <exception cref="NotFoundException"></exception>
    static string RetrieveText(int id)
    {
        if (id <= 0 || id > Texts.Count)
        {
            throw new NotFoundException($"404 - Not found: {id}");
        }
        return Texts[id - 1];
    }

    static void RotateBack<T>(List<T> vals)
    {
        // if (vals.Count == 0) return;
        var first = vals[0];
        for (int i = 1; i < vals.Count; i++)
        {
            vals[i - 1] = vals[i];
        }
        vals[^1] = first;
    }

    static string RunesToString(List<Rune> runes) => string.Concat(runes);

    static List<Rune> StringToRunes(string text) =>
        text.EnumerateRunes().ToList();
}

class NotFoundException : Exception
{
    public NotFoundException(string message) : base(message) { }
}
