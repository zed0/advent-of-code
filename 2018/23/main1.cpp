#include <iostream>
#include <vector>
#include <regex>
#include <cmath>
#include <sstream>
#include <algorithm>

struct nanobot
{
	int64_t x;
	int64_t y;
	int64_t z;
	int64_t r;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << x << "," << y << "," << z << ": " << r;
		return ss.str();
	}

	int64_t distance_to(const nanobot& other)
	{
		return std::abs(x - other.x) + std::abs(y - other.y) + std::abs(z - other.z);
	}
};

int main()
{
	std::vector<nanobot> bots;

	std::string line;
	while(std::getline(std::cin, line))
	{
		std::smatch m;
		std::regex bot_regex(R"regex(pos=<([\d-]+),([\d-]+),([\d-]+)>, r=([\d-]+))regex");
		std::regex_match(line, m, bot_regex);

		nanobot new_nanobot;
		new_nanobot.x = std::stoi(m[1]);
		new_nanobot.y = std::stoi(m[2]);
		new_nanobot.z = std::stoi(m[3]);
		new_nanobot.r = std::stoi(m[4]);
		bots.push_back(new_nanobot);
	}

	/*
	for(const auto& bot : bots)
	{
		std::cout << bot.to_string() << std::endl;
	}
	*/

	auto comp_radius = [](const nanobot& a, const nanobot& b){return a.r < b.r;};
	const auto max = std::max_element(bots.begin(), bots.end(), comp_radius);
	std::cout << max->to_string() << std::endl;

	int64_t total = 0;
	for(const auto& bot : bots)
	{
		if(max->distance_to(bot) <= max->r)
			++total;
	}

	std::cout << total << " within radius" << std::endl;

	return 0;
}
