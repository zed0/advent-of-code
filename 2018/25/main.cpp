#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <set>
#include <string>
#include <sstream>

struct star_type
{
	int a;
	int b;
	int c;
	int d;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << a << ", " << b << ", " << c << ", " << d;
		return ss.str();
	}
};

int distance(const star_type& a, const star_type& b)
{
	return std::abs(a.a - b.a) + std::abs(a.b - b.b) + std::abs(a.c - b.c) + std::abs(a.d - b.d);
}

int main()
{
	char comma;

	std::vector<star_type> stars;

	star_type new_star;
	while(std::cin >> new_star.a >> comma >> new_star.b >> comma >> new_star.c >> comma >> new_star.d)
	{
		stars.push_back(new_star);
	}

	/*
	for(const auto& star : stars)
	{
		std::cout << star.to_string() << std::endl;
	}
	*/

	std::map<int, std::vector<star_type>> constellations;

	int id = 0;
	for(const auto& star : stars)
	{
		id++;

		std::set<int> matching;

		for(const auto& constellation : constellations)
		{
			for(const auto& s : constellation.second)
			{
				if(distance(s, star) <= 3)
				{
					matching.insert(constellation.first);
				}
			}
		}

		if(matching.empty())
		{
			constellations[id].push_back(star);
		}
		else
		{
			for(auto it = matching.begin(); it != matching.end(); ++it)
			{
				auto first = *matching.begin();
				if(it == matching.begin())
				{
					constellations[first].push_back(star);
				}
				else
				{
					for(const auto& s : constellations[*it])
					{
						constellations[first].push_back(s);
					}
					constellations[*it].clear();
				}
			}
		}
	}

	int total = 0;

	for(const auto& constellation : constellations)
	{
		std::cout << constellation.first << ":" << std::endl;
		for(const auto& star : constellation.second)
		{
			std::cout << "\t" << star.to_string() << std::endl;
		}
		if(constellation.second.size() > 0)
		{
			total++;
		}
	}

	std::cout << total << " constellations" << std::endl;
}
