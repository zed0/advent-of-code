#include <iostream>
#include <iomanip>
#include <deque>
#include <cmath>
#include <algorithm>
#include <vector>
#include <sstream>
#include <limits>

struct position
{
	int64_t x;
	int64_t y;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << x << ", " << y;
		return ss.str();
	}
};

std::vector<position> directions()
{
	return {
		         {-1, 0},
		{ 0, -1},        { 0, 1},
		         { 1, 0}
	};
}


enum class equipment
{
	torch,
	climbing_gear,
	neither
};

struct path_node
{
	position pos;
	equipment equiped;
	int64_t cost = 0;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << pos.to_string();
		ss << " c=" << cost;
		switch(equiped)
		{
			case equipment::torch:
				ss << " (torch)";
				break;
			case equipment::climbing_gear:
				ss << " (climbing gear)";
				break;
			case equipment::neither:
				ss << " (neither)";
				break;
		}
		return ss.str();
	}
};

struct properties
{
	int64_t geologic;
	int64_t erosion;
	int type;

	std::string to_string() const
	{
		switch(type % 3)
		{
			case 0:
				return ".";
			case 1:
				return "=";
			case 2:
				return "|";
			default:
				throw std::runtime_error("unexpected type");
		}
	}
};

std::vector<std::vector<std::vector<path_node>>> find_path(const std::vector<std::vector<properties>>& grid, const position& target)
{
	std::vector<std::vector<std::vector<path_node>>> best_costs(grid.size(), std::vector<std::vector<path_node>>(grid[0].size()));

	std::deque<path_node> to_explore;

	path_node p;
	p.pos.x = target.x;
	p.pos.y = target.y;
	p.equiped = equipment::torch;

	best_costs[target.y][target.x].push_back(p);

	to_explore.push_back(p);

	int64_t tick = 0;
	int64_t max_target_cost = (target.x + 1 + target.y + 1) * 8;

	auto comp_pos = [](const path_node& a, const path_node& b){
		return a.pos.y + a.pos.x < b.pos.y + b.pos.x;
	};
	while(!to_explore.empty())
	{
		/*
		for(const auto& p : to_explore)
		{
			std::cout << "(" << p.pos.x << "," << p.pos.y << "), ";
		}
		std::cout << std::endl;
		*/
		//std::cout << "len " << to_explore.size() << std::endl;

		if(tick % 100000 == 0)
		{
			std::cout << "left to explore: " << to_explore.size() << std::endl;
		}
		++tick;

		auto current = to_explore.front();
		//std::cout << "exploring from << " << current.pos.x << ", " << current.pos.y << std::endl;
		to_explore.pop_front();
		//std::cout << current.to_string() << std::endl;

		if(current.cost >= max_target_cost)
		{
			continue;
		}
		int64_t max_remaining = (current.pos.x + current.pos.y) * 8;
		if(max_remaining + current.cost < max_target_cost)
		{
			//std::cout << "reducing max_target_cost to " << max_remaining + current.cost << std::endl;
			max_target_cost = max_remaining + current.cost;
		}

		for(const auto direction : directions())
		{
			path_node next;
			next.pos = {current.pos.x + direction.x, current.pos.y + direction.y};

			if(next.pos.y < 0 || next.pos.y >= grid.size())
				continue;
			if(next.pos.x < 0 || next.pos.x >= grid[next.pos.y].size())
				continue;

			const auto next_type = (grid[next.pos.y][next.pos.x].type);
			const auto current_type = (grid[current.pos.y][current.pos.x].type);

			//rocky  = 0
			//wet    = 1
			//narrow = 2

			next.cost = current.cost + 1;
			if(current.equiped == equipment::neither && next_type == 0)
			{
				next.cost += 7;
				if(current_type == 1)
					next.equiped = equipment::climbing_gear;
				else
					next.equiped = equipment::torch;
			}
			else if(current.equiped == equipment::torch && next_type == 1)
			{
				next.cost += 7;
				if(current_type == 0)
					next.equiped = equipment::climbing_gear;
				else
					next.equiped = equipment::neither;
			}
			else if(current.equiped == equipment::climbing_gear && next_type == 2)
			{
				next.cost += 7;
				if(current_type == 0)
					next.equiped = equipment::torch;
				else
					next.equiped = equipment::neither;
			}
			else
			{
				next.equiped = current.equiped;
			}

			if(next.pos.x == 0 && next.pos.y == 0)
			{
				if(next.equiped != equipment::torch)
				{
					next.cost += 7;
					next.equiped = equipment::torch;
				}
			}

			if(next.cost > max_target_cost)
			{
				//std::cout << "too big" << std::endl;
				continue;
			}

			//std::cout << "\t" << next.to_string() << std::endl;

			std::vector<path_node>& cell_costs = best_costs[next.pos.y][next.pos.x];
			auto strictly_better = [&next](const path_node& other){
				return other.cost + 7 <= next.cost || (other.equiped == next.equiped && other.cost <= next.cost);
			};
			auto strictly_worse = [&next](const path_node& other){
				return other.cost >= next.cost + 7 || (other.equiped == next.equiped && other.cost >= next.cost);
			};

			auto better = std::find_if(cell_costs.begin(), cell_costs.end(), strictly_better);
			if(better != cell_costs.end())
			{
				//std::cout << "\tstrictly better: " << better->to_string() << " vs " << next.to_string() << std::endl;
			}
			else
			{
				cell_costs.erase(std::remove_if(cell_costs.begin(), cell_costs.end(), strictly_worse), cell_costs.end());
				cell_costs.push_back(next);
				to_explore.push_back(next);
			}
			if(cell_costs.size() > 2)
			{
				throw std::runtime_error("something went wrong");
			}
		}

		/*
		for(const auto& p : to_explore)
		{
			std::cout << "(" << p.pos.x << "," << p.pos.y << "), ";
		}
		std::cout << std::endl;
		*/
	}

	return best_costs;
}

int main()
{
	int64_t depth;
	position target;

	std::string d;
	char comma;
	std::cin >> d >> depth;
	std::cin >> d >> target.x >> comma >> target.y;

	std::cout << depth << std::endl;
	std::cout << target.x << ", " << target.y << std::endl;

	position grid_size = {target.x * 20, target.y * 2};
	//position grid_size = {target.x, target.y};

	std::vector<std::vector<properties>> grid(grid_size.y+1, std::vector<properties>(grid_size.x+1));

	int64_t risk_level = 0;

	for(int64_t y=0; y<=grid_size.y; ++y)
	{
		for(int64_t x=0; x<=grid_size.x; ++x)
		{
			properties props;
			if(x == target.x && y == target.y)
			{
				props.geologic = 0;
			}
			else if(y == 0)
			{
				props.geologic = x * 16807;
			}
			else if(x == 0)
			{
				props.geologic = y * 48271;
			}
			else
			{
				props.geologic = grid[y-1][x].erosion * grid[y][x-1].erosion;
			}

			props.erosion = (props.geologic + depth) % 20183;
			props.type = props.erosion % 3;
			grid[y][x] = props;

			risk_level += props.type;
		}
	}

	/*
	for(const auto& row : grid)
	{
		for(const auto& cell : row)
		{
			std::cout << cell.to_string();
		}
		std::cout << std::endl;
	}
	*/

	std::cout << "risk level: " << risk_level << std::endl;

	auto best_costs = find_path(grid, target);

	auto comp = [](const path_node& a, const path_node& b){return a.cost < b.cost;};

	/*
	for(int64_t y=0; y<=grid_size.y; ++y)
	{
		for(int64_t x=0; x<=grid_size.x; ++x)
		{
			auto& cell = best_costs[y][x];
			auto min = std::min_element(cell.begin(), cell.end(), comp);

			if(x == target.x && y == target.y)
				std::cout << "T";
			else
				std::cout << " ";

			if(min != cell.end())
			{
				std::cout << std::setfill(' ') << std::setw(2) << min->cost;
			}
			else
			{
				std::cout << " ?";
			}
		}
		std::cout << std::endl;
	}
	*/

	auto& best_at_target = best_costs[0][0];
	auto min = std::min_element(best_at_target.begin(), best_at_target.end(), comp);
	std::cout << "Best at target: " << min->cost << std::endl;

	return 0;
}
