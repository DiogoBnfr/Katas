namespace split_strings;

internal class Program
{
    private static void Main(string[] args)
    {
        string[] arr = split_string("abcdef");

        foreach (var str in arr)
        {
            Console.WriteLine(str);
        }
    }

    private static string[] split_string(string str)
    {
        var arr = new string[(int)Math.Floor((str.Length - 1) / 2.0 + 1)];

        for (int i = 0, j = 0; j < arr.Length; i += 2, j++)
        {
            if (i == str.Length - 1)
            {
                arr[j] = $"{str[i]}_";
                break;
            }
            arr[j] = $"{str[i]}{str[i + 1]}";
        }
        return arr;
    }
}
