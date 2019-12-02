#include <iostream>
#include <vector>
#include <map>
#include <string>
#include <regex>
#include <algorithm>
#include <sstream>

enum class material
{
	sand,
	clay,
	moving,
	still,
	source
};

material char_to_material(char c)
{
	if(c == '.') return material::sand;
	if(c == '#') return material::clay;
	if(c == '|') return material::moving;
	if(c == '~') return material::still;
	if(c == '+') return material::source;

	throw std::runtime_error("invalid material");
}

char material_to_char(material m)
{
	if(m == material::sand  ) return '.';
	if(m == material::clay  ) return '#';
	if(m == material::moving) return '|';
	if(m == material::still ) return '~';
	if(m == material::source) return '+';

	throw std::runtime_error("invalid material");
}

std::string material_to_string(material m)
{
	if(m == material::sand  ) return "sand";
	if(m == material::clay  ) return "clay";
	if(m == material::moving) return "moving";
	if(m == material::still ) return "still";
	if(m == material::source) return "source";
	throw std::runtime_error("invalid material");
}

bool read_rect(
	std::vector<std::vector<material>>& grid,
	size_t& max_x,
	size_t& min_x,
	size_t& max_y,
	size_t& min_y
)
{
	std::string line;
	if(!std::getline(std::cin, line))
		return false;

	std::smatch m;
	std::regex rect_regex(R"regex((.)=(\d+), (.)=(\d+)\.\.(\d+))regex");
	std::regex_match(line, m, rect_regex);

	size_t start = static_cast<size_t>(std::stoi(m[4]));
	size_t end = static_cast<size_t>(std::stoi(m[5]));
	for(size_t i=start; i<=end; ++i)
	{
		size_t x=0;
		size_t y=0;
		if(m[1] == "x")
		{
			x = static_cast<size_t>(std::stoi(m[2]));
			y = i;
		}
		else
		{
			x = i;
			y = static_cast<size_t>(std::stoi(m[2]));
		}

		if(x > max_x)
			max_x = x;

		if(y > max_y)
			max_y = y;

		if(x < min_x)
			min_x = x;

		if(y < min_y)
			min_y = y;

		grid.resize(max_y+1);
		for(auto& row : grid)
			row.resize(max_x+2, material::sand);

		grid[y][x] = material::clay;
	}

	return true;
}

void print_grid(
	const std::vector<std::vector<material>>& grid,
	const size_t max_x,
	const size_t min_x,
	const size_t max_y,
	const size_t min_y
)
{
	for(size_t y=min_y; y<=max_y; ++y)
	{
		for(size_t x=min_x-1; x<=max_x+1; ++x)
		{
			std::cout << material_to_char(grid[y][x]);
		}
		std::cout << "\n";
	}
	std::cout << std::endl;
}

struct position
{
	position(size_t x_, size_t y_) :
		x(x_),
		y(y_)
	{
	}

	size_t x;
	size_t y;
};

int32_t main()
{
	std::vector<std::vector<material>> grid;

	size_t max_x=0;
	size_t max_y=0;
	size_t min_x=99999;
	size_t min_y=99999;

	while(read_rect(grid, max_x, min_x, max_y, min_y))
	{
	}

	grid[0][500] = material::source;
	std::deque<position> active_cells;
	active_cells.emplace_back(500, 1);

	std::vector<int32_t> directions{1, -1};

	int round = 0;

	while(active_cells.size())
	{
		round++;
		const auto cell = active_cells.front();
		active_cells.pop_front();

		if(grid[cell.y][cell.x] == material::still)
			continue;

		grid[cell.y][cell.x] = material::moving;

		if(cell.y >= max_y)
			continue;

		if(grid[cell.y+1][cell.x] == material::sand)
		{
			active_cells.emplace_back(cell.x, cell.y+1);
		}
		else if(grid[cell.y+1][cell.x] == material::clay || grid[cell.y+1][cell.x] == material::still)
		{
			bool flow = false;
			for(const int32_t direction : directions)
			{
				position current(cell.x, cell.y);
				while(true)
				{
					current.x = static_cast<size_t>(static_cast<int32_t>(current.x) + direction);
					if(grid[current.y][current.x] == material::clay)
					{
						break;
					}

					grid[current.y][current.x] = material::moving;

					const auto below = grid[current.y+1][current.x];
					if(below == material::sand || below == material::moving)
					{
						active_cells.push_back(current);
						flow = true;
						break;
					}
				}
			}

			if(!flow)
			{
				for(const int32_t direction : directions)
				{
					position current(cell.x, cell.y);
					while(grid[current.y][current.x] == material::moving || grid[current.y][current.x] == material::still)
					{
						grid[current.y][current.x] = material::still;
						current.x = static_cast<size_t>(static_cast<int32_t>(current.x) + direction);
					}
				}

				for(size_t x=min_x-1; x<=max_x+1; ++x)
				{
					if(grid[cell.y-1][x] == material::moving)
						active_cells.emplace_back(x, cell.y-1);
				}
			}
		}

		//if(round%100 == 0)
			//print_grid(grid, max_x, min_x, max_y, min_y);
	}

	print_grid(grid, max_x, min_x, max_y, min_y);

	std::map<material, int32_t> counts;
	for(size_t y=min_y; y<=max_y; ++y)
	{
		for(size_t x=min_x-1; x<=max_x+1; ++x)
		{
			counts[grid[y][x]]++;
		}
	}

	for(const auto& count : counts)
	{
		std::cout << material_to_string(count.first) << ": " << count.second << std::endl;
	}

	std::cout << "moving + still = " << counts[material::moving] + counts[material::still] << std::endl;
	return 0;
}
