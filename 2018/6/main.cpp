#include <iostream>
#include <iomanip>
#include <cmath>
#include <vector>
#include <set>
#include <map>
#include <algorithm>

struct coordinate
{
	int x;
	int y;
	int id;
	int count;

	coordinate() :
		count(0)
	{
	}

	int distance(const coordinate& other) const
	{
		return std::abs(x - other.x) + std::abs(y - other.y);
	}
};

int main()
{
	std::map<int, coordinate> input;

	coordinate input_coord;
	char comma;
	int id=1;
	while(std::cin >> input_coord.x >> comma >> input_coord.y)
	{
		input_coord.id = id++;
		input[id] = input_coord;
	}

	auto coordinates = input;
	auto coordinates2 = input;

	/*
	for(const auto& coord : coordinates)
	{
		std::cout << coord.second.id << ": " << coord.second.x << ", " << coord.second.y << std::endl;
	}
	*/

	auto comp_x = [](const std::pair<int, coordinate>& a, const std::pair<int, coordinate>& b){return a.second.x < b.second.x;};
	auto comp_y = [](const std::pair<int, coordinate>& a, const std::pair<int, coordinate>& b){return a.second.y < b.second.y;};

	int max_x = std::max_element(coordinates.begin(), coordinates.end(), comp_x)->second.x;
	int max_y = std::max_element(coordinates.begin(), coordinates.end(), comp_y)->second.y;

	std::vector<std::vector<int>> grid;
	grid.resize(max_y+1, std::vector<int>(max_x+1, 0));

	for(int y=0; y<=max_y; ++y)
	{
		for(int x=0; x<=max_x; ++x)
		{
			coordinate current_coord;
			current_coord.x = x;
			current_coord.y = y;

			int best_distance = 9999999;
			int best_id = 0;
			bool tying = false;
			for(const auto& coord : coordinates)
			{
				int distance = coord.second.distance(current_coord);
				if(distance == best_distance)
				{
					tying = true;
				}
				else if(distance < best_distance)
				{
					tying = false;
					best_distance = distance;
					best_id = coord.second.id;
				}
			}
			if(!tying)
				grid[y][x] = best_id;
		}
	}

	std::set<int> infinite_ids;
	for(int y=0; y<=max_y; ++y)
	{
		coordinates.erase(grid[y][0]);
		coordinates.erase(grid[y][max_x]);
	}

	for(int x=0; x<=max_x; ++x)
	{
		coordinates.erase(grid[0][x]);
		coordinates.erase(grid[max_y][x]);
	}

	for(const auto& row : grid)
	{
		for(const auto& cell : row)
		{
			if(coordinates.count(cell))
				coordinates[cell].count++;
		}
	}

	/*
	for(const auto& coord : coordinates)
	{
		std::cout << coord.second.id << ": " << coord.second.x << ", " << coord.second.y << ": " << coord.second.count << std::endl;
	}
	*/

	auto max_count = [](const std::pair<int, coordinate>& a, const std::pair<int, coordinate>& b){return a.second.count < b.second.count;};
	auto largest = std::max_element(coordinates.begin(), coordinates.end(), max_count);
	std::cout << "largest count: " << largest->second.id << "(" << largest->second.count << ")" << std::endl;

	/*
	for(const auto& row : grid)
	{
		for(const auto& cell : row)
		{
			std::cout << std::setfill('0') << std::setw(2) << cell << " ";
		}
		std::cout << std::endl;
	}
	*/

	int region_size = 0;

	for(int y=0; y<=max_y; ++y)
	{
		for(int x=0; x<=max_x; ++x)
		{
			coordinate current_coord;
			current_coord.x = x;
			current_coord.y = y;

			int total_distance = 0;
			for(const auto& coord : coordinates2)
			{
				total_distance += coord.second.distance(current_coord);
				if(total_distance >= 10000)
					break;
			}

			if(total_distance < 10000)
				++region_size;
		}
	}

	std::cout << "region within 10000 of all: " << region_size << std::endl;

	return 0;
}
