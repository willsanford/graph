#include <cstdlib>
#include <vector>
#include <iostream>
#include <assert.h>
#include "point.hpp"
#include "segment.hpp"
#include "bentley_ottmann.hpp"

// Define our template class for the red black tree
int main(){
    RedBlackTree<int>* tree = new RedBlackTree<int>;

    for (int i = 1; i < 15; i++){
        tree->insert( i);
    }
    std::vector<Node<int>*> nodes = tree->getAllNodes();

    for (Node<int>* node: nodes){
        std::cout << node->getData() << std::endl;
    }

    delete tree;
    return 0;
}