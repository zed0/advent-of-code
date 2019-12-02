#include <iostream>
#include <vector>
#include <sstream>
#include <string>
#include <algorithm>

struct elf
{
	size_t index;
	int current_score;
};

int main()
{
	uint64_t num_recipes = 323081;
	//uint32_t num_recipes = 51589;
	//uint32_t num_recipes = 59414;

	std::basic_string<int> recipes = {3, 7};
	std::vector<elf> elves;
	elves.emplace_back();
	elves.back().index = 0;
	elves.emplace_back();
	elves.back().index = 1;

	//while(recipes.size() < num_recipes + 10)
	std::basic_string<int> needle;
	{
		auto n = num_recipes;
		while(n > 0)
		{
			needle.push_back(n % 10);
			n /= 10;
		}
		std::reverse(needle.begin(), needle.end());
	}

	while(recipes.size() < needle.size() || recipes.find(needle, recipes.size() - needle.size() - 1) == std::string::npos)
	{
		int32_t total_score = 0;
		for(auto& elf : elves)
		{
			elf.current_score = recipes[elf.index];
			total_score += elf.current_score;
		}

		std::vector<int> digits;
		if(total_score == 0)
		{
			digits.push_back(0);
		}
		else
		{
			while(total_score > 0)
			{
				digits.push_back(total_score % 10);
				total_score /= 10;
			}
		}
		recipes.insert(recipes.end(), digits.rbegin(), digits.rend());

		for(auto& elf : elves)
		{
			elf.index = (elf.index + elf.current_score + 1) % recipes.size();
		}

		/*
		for(const auto& recipe : recipes)
		{
			std::cout << recipe;
		}
		std::cout << std::endl;
		*/
	}

	auto pos = recipes.find(needle, recipes.size() - needle.size() - 1);
	std::cout << pos << std::endl;

	/*
	for(auto it = recipes.begin() + num_recipes; it != recipes.begin() + num_recipes + 10; ++it)
	{
		std::cout << *it;
	}
	std::cout << std::endl;
	*/
	//std::cout << recipes.size() << std::endl;

	return 0;
}
