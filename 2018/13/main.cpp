#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <algorithm>

class cart_collision_error : public std::runtime_error
{
public:
	int x;
	int y;
	cart_collision_error(int x_, int y_) :
		std::runtime_error("Cart collision"),
		x(x_),
		y(y_)
	{
	}
};

int char_to_direction(char d)
{
	if(d == '<') return 0;
	if(d == '^') return 1;
	if(d == '>') return 2;
	if(d == 'v') return 3;

	std::cout << "Unexpected: '" << d << "'" << std::endl;
	throw std::runtime_error("Unexpected character");
}

char direction_to_char(int d)
{
	if(d == 0) return '<';
	if(d == 1) return '^';
	if(d == 2) return '>';
	if(d == 3) return 'v';
	throw std::runtime_error("Unexpected direction");
}

struct cart_type
{
	int id;
	int x;
	int y;
	int direction;
	bool dead = false;
	int next = -1;

	void move(const std::vector<std::vector<char>>& track)
	{
		switch(direction_to_char(direction))
		{
			case '^':
				--y;
				if(track[y][x] == '\\') direction = char_to_direction('<');
				if(track[y][x] == '/')  direction = char_to_direction('>');
				if(track[y][x] == '+'){direction = (direction + next + 4)%4; ++next; if(next == 2)next = -1;}
				break;
			case 'v':
				++y;
				if(track[y][x] == '\\') direction = char_to_direction('>');
				if(track[y][x] == '/')  direction = char_to_direction('<');
				if(track[y][x] == '+'){direction = (direction + next + 4)%4; ++next; if(next == 2)next = -1;}
				break;
			case '<':
				--x;
				if(track[y][x] == '\\') direction = char_to_direction('^');
				if(track[y][x] == '/')  direction = char_to_direction('v');
				if(track[y][x] == '+'){direction = (direction + next + 4)%4; ++next; if(next == 2)next = -1;}
				break;
			case '>':
				++x;
				if(track[y][x] == '\\') direction = char_to_direction('v');
				if(track[y][x] == '/')  direction = char_to_direction('^');
				if(track[y][x] == '+'){direction = (direction + next + 4)%4; ++next; if(next == 2)next = -1;}
				break;
		}
	}
};

bool order_carts(const cart_type& a, const cart_type& b)
{
	return (a.y * 1000 + a.x) < (b.y * 1000 + b.x);
}

void print(const std::vector<std::vector<char>>& track, const std::vector<cart_type>& carts)
{
	int y=0;
	for(const auto& row : track)
	{
		int x=0;
		for(const auto& cell : row)
		{
			bool has_cart = false;
			for(const auto& cart : carts)
			{
				if(cart.dead)
					continue;
				if(cart.x == x && cart.y == y)
				{
					has_cart = true;
					std::cout << direction_to_char(cart.direction);
				}
			}
			if(!has_cart)
				std::cout << cell;
			++x;
		}
		std::cout << std::endl;
		++y;
	}
}

void tick(const std::vector<std::vector<char>>& track, std::vector<cart_type>& carts, bool remove_carts)
{
	std::sort(carts.begin(), carts.end(), order_carts);

	for(auto& cart : carts)
	{
		if(cart.dead)
			continue;

		cart.move(track);
		auto is_collision = [&cart](const cart_type& other){
			return !other.dead && cart.x == other.x && cart.y == other.y && cart.id != other.id;
		};
		const auto collision = std::find_if(carts.begin(), carts.end(), is_collision);
		if(collision != carts.end())
		{
			std::stringstream error_text;
			if(!remove_carts)
				throw cart_collision_error(cart.x, cart.y);

			cart.dead = true;
			collision->dead = true;
		}
	}
}

int main()
{
	std::vector<std::vector<char>> tracks;
	std::vector<cart_type> carts;

	{
		int id=0;
		int y=0;
		std::string line;
		while(std::getline(std::cin, line))
		{
			tracks.emplace_back();

			int x=0;
			for(const auto rail : line)
			{
				if(rail == '^' || rail == 'v')
				{
					cart_type new_cart;
					new_cart.id = id++;
					new_cart.x = x;
					new_cart.y = y;
					new_cart.direction = char_to_direction(rail);
					carts.push_back(new_cart);

					tracks.back().push_back('|');
				}
				else if(rail == '>' || rail == '<')
				{
					cart_type new_cart;
					new_cart.id = id++;
					new_cart.x = x;
					new_cart.y = y;
					new_cart.direction = char_to_direction(rail);
					carts.push_back(new_cart);

					tracks.back().push_back('-');
				}
				else
				{
					tracks.back().push_back(rail);
				}
				++x;
			}

			++y;
		}
	}

	//print(tracks, carts);

	{
		std::cout << "Part 1: " << std::endl;
		auto carts1 = carts;
		int i=0;
		try
		{
			while(true)
			{
				tick(tracks, carts1, false);
				//print(tracks, carts1);
				++i;
			}
		}
		catch(const cart_collision_error& e)
		{
			std::cout << "ended on tick " << i << " at " << e.x << "," << e.y << std::endl;
			//print(tracks, carts1);
		}
	}

	{
		std::cout << "Part 2: " << std::endl;
		auto carts2 = carts;
		int i=0;
		auto is_alive = [](const cart_type& cart){return !cart.dead;};
		while(std::count_if(carts2.begin(), carts2.end(), is_alive) > 1)
		{
			tick(tracks, carts2, true);
			//print(tracks, carts2);
			++i;
		}
		const auto remaining = std::find_if(carts2.begin(), carts2.end(), is_alive);
		std::cout << "ended on tick " << i-1 << " with " << remaining->x << "," << remaining->y << std::endl;
	}

	/*
	for(const auto& cart : carts)
	{
		std::cout << cart.x << ", " << cart.y << ": " << direction_to_char(cart.direction) << std::endl;
	}
	*/

	return 0;
}
