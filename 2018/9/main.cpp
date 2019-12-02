#include <iostream>
#include <list>
#include <map>
#include <algorithm>

void print_marbles(const std::list<int>::iterator& it, const std::list<int>& marbles, const int64_t& player)
{
	std::cout << player << ": ";
	for(auto itt = marbles.begin(); itt != marbles.end(); ++itt)
	{
		std::cout << *itt;
		if(itt == it)
			std::cout << "*";
		else
			std::cout << " ";
	}
	std::cout << "x";
	if(it == marbles.end())
		std::cout << "*";

	std::cout << std::endl;
}

void inc_iterator(std::list<int>::iterator& it, int n, std::list<int>& l)
{
	for(int i=0; i<n; ++i)
	{
		++it;
		if(it == l.end())
			it = l.begin();
	}
}

void dec_iterator(std::list<int>::iterator& it, int n, std::list<int>& l)
{
	for(int i=0; i<n; ++i)
	{
		--it;
		if(it == l.begin())
			it = l.end();
	}
}

int main()
{
	int num_players;
	int64_t max_points;

	std::string players, last, marble, is, worth;

	std::cin >> num_players >> players >> last >> marble >> is >> worth >> max_points;

	max_points *= 100;

	std::cout << num_players << " players, " << max_points << " max points" << std::endl;
	std::map<int64_t, int64_t> points;

	std::list<int> marbles{0};
	auto it = marbles.begin();
	for(int64_t current_points = 1; current_points <= max_points; ++current_points)
	{
		if(current_points % 23)
		{
			inc_iterator(it, 1, marbles);
			++it;
			it = marbles.insert(it, current_points);
		}
		else
		{
			int64_t player = ((current_points - 1) % num_players) + 1;

			dec_iterator(it, 7, marbles);
			points[player] += current_points;
			points[player] += *it;
			it = marbles.erase(it);
		}
	}

	auto comp_points = [](const std::pair<int64_t, int64_t>& a, const std::pair<int64_t, int64_t>& b){return a.second < b.second;};
	auto max = std::max_element(points.begin(), points.end(), comp_points);
	std::cout << "Best: " << max->first << ": " << max->second << std::endl;

	return 0;
}
