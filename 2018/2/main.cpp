#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>
#include <functional>

int main()
{
	std::vector<std::string> inputs;
	std::string item;
	while(std::cin >> item)
	{
		inputs.push_back(item);
	}

	{
		size_t twos = 0;
		size_t threes = 0;

		for(const auto& input : inputs)
		{
			std::map<char, size_t> counts;

			for(const auto c : input)
			{
				counts[c]++;
			}

			bool has_two = false;
			bool has_three = false;
			for(const auto& count : counts)
			{
				if(count.second == 2)
					has_two = true;
				if(count.second == 3)
					has_three = true;
			}

			twos += has_two;
			threes += has_three;
		}

		std::cout << twos * threes << "(" << twos << " * " << threes << ")" << std::endl;
	}

	{
		for(const auto& input1 : inputs)
		{
			for(const auto& input2 : inputs)
			{
				int non_matching = std::inner_product(
					input1.begin(),
					input1.end(),
					input2.begin(),
					0,
					std::plus<>(),
					std::not_equal_to<>()
				);

				if(non_matching == 1)
				{
					std::string intersection;
					for(size_t i=0; i<input1.size(); ++i)
					{
						if(input1[i] == input2[i])
							intersection += input1[i];
					}
					std::cout << intersection << " (" << input1 << ", " << input2 << ")" << std::endl;
					return 0;
				}
			}
		}
	}

	return 0;
}
