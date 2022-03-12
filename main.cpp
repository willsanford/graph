#include <cstdlib>
#include <vector>
#include <iostream>
#include <assert.h>
#include "point.hpp"
#include "segment.hpp"
#include "bentley_ottmann.hpp"

// Define our template class for the red black tree
int main(){
    RedBlackTree<float> tree;

    for (float i = 1; i < 15; i++){
        tree.insert((float) i);
    }
    tree.printTree();
    return 0;
}