using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode2021.D22
{
    class D22P1
    {
        public static void Run()
        {
            string raw_data_string = @"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
            Point.Initialize();
            Range.Initialize();

            Console.WriteLine(Point.collection.Count.ToString());
            foreach (string raw_data in raw_data_string.Split('\n'))
            {
                Console.WriteLine(raw_data);

                bool state = raw_data.Split(" ")[0] == "on";
                List<string> rangeData = raw_data.Split(" ")[1].Split(",").ToList();
                Range xRange = Range.fetchFromData(rangeData[0]);
                Range yRange = Range.fetchFromData(rangeData[0]);
                Range zRange = Range.fetchFromData(rangeData[0]);

                foreach(int z in zRange.range)
                {
                    Console.Write(z);
                    foreach(int y in yRange.range)
                    {
                        foreach (int x in zRange.range)
                        {
                            Point result = Point.fetchFrom(new Vector3(x, y, z));
                            if (result != null)
                            {
                                result.active = state;
                            }
                        }
                    }
                    Console.WriteLine();
                }
            }
            Console.WriteLine(Point.collection.Where(n => n.active).Count());
        }
    }

    class Point
    {
        public static List<Point> collection;

        public static void Initialize()
        {
            collection = new List<Point>();
            for(int z = -50; z <= 50; z++)
            {
                for (int y = -50; y <= 50; y++)
                {
                    for (int x = -50; x <= 50; x++)
                    {
                        collection.Add(new Point(new Vector3(x, y, z), false));
                    }
                }
            }
        }

        public Vector3 position;
        public bool active;

        public Point(Vector3 _position, bool _active)
        {
            position = _position;
            active = _active;
        }

        public static Point fetchFrom(Vector3 position)
        {
            foreach(Point point in Point.collection)
            {
                if(point.position == position)
                {
                    return point;
                }
            }
            return null;
        }
    }

    class Vector3
    {
        public int x, y, z;

        public Vector3(int _x, int _y, int _z)
        {
            x = _x;
            y = _y;
            z = _z;
        }
    }

    class Range
    {
        public static Range defaultRange;

        public static void Initialize()
        {
            defaultRange = new Range(-50, 50);
        }

        public int start, end;
        public List<int> range;

        public Range(int _start, int _end)
        {
            start = _start;
            end = _end;

            range = new List<int>();
            for(int x = start; x <= end; x++)
            {
                range.Add(x);
            }
        }

        public static Range fetchFromData(string data)
        {
            List<string> processed_data = data.Split("=")[1].Split("..").ToList();
            int start = int.Parse(processed_data[0]);
            int end = int.Parse(processed_data[1]);

            return new Range(start, end);
        }

        public bool inRange(int item)
        {
            // start <= item <= end
            return (start <= item) && (item <= end);
        }
    }
}
