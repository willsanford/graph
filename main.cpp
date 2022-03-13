#include <cstdlib>
#include <vector>
#include <iostream>
#include <assert.h>
#include "point.hpp"
#include "segment.hpp"
#include "bentley_ottmann.hpp"

// Define our template class for the red black tree
int main(){
    RedBlackTree<int> tree;

    for (int i = 1; i < 15; i++){
        tree.insert( i);
    }
    tree.printTree();

    Node<int>* node = tree.searchTree(2);
    std::cout << node->getData() << std::endl;
    return 0;
}