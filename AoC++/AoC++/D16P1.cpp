#include <iostream>
#include <string>
#include <list>
#include <fstream>
#include "D16P1.h"

using namespace std;

class range
{
public:
    int start;
    int end;

    range(int s, int e)
    {
        start = s;
        end = e;
    }

    bool compare(int amount)
    {
        return (amount >= start) && (amount <= end);
    }
};

list<range> ranges = list<range>();
list<int> tickets = list<int>();

string input_data = "";

string range_data = "";
string ticket_data = "";

void fetch_input_data()
{
    ifstream input_file("D16.txt");
    string temp;
    while (getline(input_file, temp))
    {
        input_data += temp;
        input_data += "\n";
    }

    string batch = "";
    bool previous = false;
    bool swapped = false;
    for (char x : input_data)
    {
        if (x == '\n')
        {
            if (!swapped)
            {
                range_data += batch;
                range_data += "\n";
            }
            else
            {
                ticket_data += batch;
                ticket_data += "\n";
            }
            batch = "";
            if (!swapped)
            {
                if (previous)
                {
                    swapped = true;
                    range_data += batch;
                }
                previous = true;
            }
        }
        else
        {
            batch += x;
            previous = false;
        }
    }
    ticket_data += batch;
}

void fetch_range_data()
{
    list<string> seperated_ranges = list<string>();
    string batch = "";
    for (char x : range_data)
    {
        if (x == '\n')
        {
            seperated_ranges.push_back(batch);
            batch = "";
        }
        else
        {
            batch += x;
        }
    }
    seperated_ranges.push_back(batch);

    for (string line : seperated_ranges)
    {
        list<string> splits = list<string>();

        batch = "";
        for (char x : line)
        {
            if (x == ' ')
            {
                splits.push_back(batch);
                batch = "";
            }
            else
            {
                batch += x;
            }
        }
        splits.push_back(batch);

        for (string x : splits)
        {
            if (x.find('-') != string::npos)
            {
                string s_start, s_end;
                bool swapped = false;
                for (char i : x)
                {
                    if (i == '-')
                    {
                        swapped = true;
                        continue;
                    }

                    if (swapped)
                    {
                        s_end += i;
                    }
                    else
                    {
                        s_start += i;
                    }
                }
                int start = stoi(s_start);
                int end = stoi(s_end);
                ranges.push_back(range(start, end));
            }
        }
    }
}

void fetch_ticket_numbers()
{
    string batch = "";
    for (char c : ticket_data)
    {
        if (isdigit(c))
        {
            batch += c;
        }
        else
        {
            if (!batch.empty())
            {
                tickets.push_back(stoi(batch));
                batch = "";
            }
        }
    }
}

void invoke()
{
    fetch_input_data();
    fetch_range_data();
    fetch_ticket_numbers();
    list<int> fails = list<int>();
    for (int ticket : tickets)
    {
        bool result = false;
        for (range r : ranges)
        {
            //cout << !r.compare(ticket) << " :: " << ticket << " | " << r.start << " - " << r.end << endl;
            if (r.compare(ticket))
            {
                result = true;
                continue;
            }
        }

        if (!result)
        {
            fails.push_back(ticket);
        }
    }
    int final_int = 0;
    for (int i : fails)
    {
        final_int += i;
    }
    cout << final_int << endl;
}
