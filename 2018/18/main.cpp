#include <iostream>
#include <thread>
#include <vector>
#include <string>
#include <map>

void print_grid(const std::vector<std::string>& grid)
{
	for(const auto& row : grid)
	{
		std::cout << row << "\n";
	}
	std::cout << std::flush;
}

struct direction{
	int x;
	int y;
};

std::vector<direction> directions{
	{-1, -1}, { 0, -1}, { 1, -1},
	{-1,  0},           { 1,  0},
	{-1,  1}, { 0,  1}, { 1,  1}
};

int adjacent_of_state(int x, int y, char state, const std::vector<std::string>& grid)
{
	int total = 0;

	for(const auto& direction : directions)
	{
		int current_x = x + direction.x;
		int current_y = y + direction.y;
		if(current_y >= 0 && current_y < grid.size())
		{
			if(current_x >= 0 && current_x < grid[current_y].size())
			{
				if(grid[current_y][current_x] == state)
					++total;
			}
		}
	}

	return total;
}

void tick(std::vector<std::string>& grid)
{
	const auto prev_grid = grid;

	for(int y=0; y<prev_grid.size(); ++y)
	{
		for(int x=0; x<prev_grid[y].size(); ++x)
		{
			switch(prev_grid[y][x])
			{
				case '.':
					if(adjacent_of_state(x, y, '|', prev_grid) >= 3)
						grid[y][x] = '|';
					break;
				case '|':
					if(adjacent_of_state(x, y, '#', prev_grid) >= 3)
						grid[y][x] = '#';
					break;
				case '#':
					if(adjacent_of_state(x, y, '#', prev_grid) == 0 || adjacent_of_state(x, y, '|', prev_grid) == 0)
						grid[y][x] = '.';
					break;
			}
		}
	}
}

void print_score(std::vector<std::string> grid)
{
	std::map<char, int> counts;
	for(const auto& row : grid)
	{
		for(const auto cell : row)
		{
			counts[cell]++;
		}
	}

	for(const auto& item : counts)
	{
		std::cout << item.first << ": " << item.second << "; ";
	}
	std::cout << "(| * # = " << counts['|']*counts['#'] << ")" << std::endl;
}

size_t hash_grid(std::vector<std::string> grid)
{
	std::string whole;
	for(const auto& row : grid)
	{
		whole += row;
	}

	return std::hash<std::string>{}(whole);
}

int main()
{
	int64_t ticks = 1000000000;
	std::vector<std::string> grid;

	std::string line;
	while(std::getline(std::cin, line))
	{
		grid.emplace_back();
		for(const char c : line)
		{
			grid.back().push_back(c);
		}
	}

	std::map<size_t, int64_t> hashes;

	//print_grid(grid);
	bool skipped = false;

	for(int64_t i=1; i<=ticks; ++i)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(20));
		std::cout << "\x1b[2J\x1b[1;1H" << std::flush;
		std::cout << i << ":" << std::endl;
		tick(grid);
		print_grid(grid);
		if(!skipped)
		{
			const auto hash = hash_grid(grid);
			if(hashes.count(hash))
			{
				int64_t cycle_length = i - hashes[hash];
				std::cout << "cycle length: " << cycle_length << std::endl;
				int64_t remaining = (ticks - i) % cycle_length;
				i = ticks - remaining;
				skipped = true;
			}
			else
			{
				hashes[hash] = i;
			}
		}
	}
	print_score(grid);
	return 0;
}
