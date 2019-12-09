#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

int main()
{
	int num_players;
	int64_t max_points;

	std::string players, last, marble, is, worth;

	std::cin >> num_players >> players >> last >> marble >> is >> worth >> max_points;

	max_points *= 100;

	std::cout << num_players << " players, " << max_points << " max points" << std::endl;
	std::map<int64_t, int64_t> points;

	std::vector<int> marbles{0};
	marbles.reserve(max_points);
	int64_t index = 0;
	for(int64_t current_points = 1; current_points <= max_points; ++current_points)
	{
		int64_t player = ((current_points - 1) % num_players) + 1;

		if(current_points % 23)
		{
			index = (index + 2) % marbles.size();
			marbles.insert(marbles.begin() + index + 1, current_points);
		}
		else
		{
			index = (index - 7 + marbles.size()) % marbles.size();
			points[player] += current_points;
			points[player] += *(marbles.begin() + index + 1);
			marbles.erase(marbles.begin() + index + 1);
		}
	}

	auto comp_points = [](const std::pair<int64_t, int64_t>& a, const std::pair<int64_t, int64_t>& b){return a.second < b.second;};
	auto max = std::max_element(points.begin(), points.end(), comp_points);
	std::cout << "Best: " << max->first << ": " << max->second << std::endl;

	return 0;
}
