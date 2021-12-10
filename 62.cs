using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;

class Program
{
    public static List<int> fishes;
    public static void Main(string[] args)
    {
        string rawDataString = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2";
        fishes = new List<int>();
        foreach(string x in rawDataString.Split(','))
        {
            fishes.Add(int.Parse(x));
        }

        for (int i = 0; i < 256; i++)
        {
            Console.WriteLine(i);
            Update();
        }

        Console.WriteLine(fishes.Count.ToString());
    }

    public static void Update()
    {
        int[] copy = fishes.ToArray();
        foreach(var i in copy.Select((value, index) => new {index, value}))
        {
            if(i.value <= 0)
            {
                fishes[i.index] = 6;
                fishes.Add(8);
            }
            else
            {
                fishes[i.index] -= 1;
            }
        }
    }
}