#include <iostream>
#include <vector>
#include <set>

int main()
{
	std::vector<int> numbers;
	int item;
	while(std::cin >>item)
	{
		numbers.push_back(item);
	}
	int total = 0;
	std::set<int> totals;
	while(true)
	{
		for(const auto item : numbers)
		{
			total += item;

			if(totals.count(total))
			{
				std::cout << total << std::endl;
				return 1;
			}
			totals.insert(total);
		}
	}

	return 0;
}
