#include <iostream>
#include <vector>

struct node_type
{
	std::vector<int> metadata;

	std::vector<node_type> children;

	int value() const
	{
		int total = 0;
		if(children.empty())
		{
			for(const auto meta : metadata)
				total += meta;
		}
		else
		{
			for(const auto meta : metadata)
			{
				if(meta == 0)
					continue;
				if(meta > children.size())
					continue;
				total += children[meta - 1].value();
			}
		}
		return total;
	}
};

std::vector<node_type> all_nodes;

node_type read_node()
{
	node_type new_node;

	int num_children;
	int num_metadata;
	std::cin >> num_children >> num_metadata;

	for(int i=0; i<num_children; ++i)
		new_node.children.push_back(read_node());

	for(int i=0; i<num_metadata; ++i)
	{
		int meta;
		std::cin >> meta;
		new_node.metadata.push_back(meta);
	}

	all_nodes.push_back(new_node);
	return new_node;
}

int main()
{
	const auto root = read_node();

	int total = 0;
	for(const auto& node : all_nodes)
	{
		for(const auto& meta : node.metadata)
		{
			total += meta;
		}
	}
	std::cout << "Part 1: " << total << std::endl;

	std::cout << "Part 2: " << root.value() << std::endl;
	return 0;
}
