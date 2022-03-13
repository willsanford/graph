#ifndef TREE_H 
#define TREE_H

#include <cstdlib>
#include <string>
#include "node.hpp"

template<typename T>
class RedBlackTree{
  private:
    Node<T> *root;

    void insertFix(Node<T>* k);
    void printHelper(Node<T>* root, int depth);
    void handleRightSituations(Node<T>* node, Node<T>* parent, Node<T>* grandparent);
    void handleLeftSituations(Node<T>* node, Node<T>* parent, Node<T>* grandparent);
    void recolorAndRotate(Node<T>* node);
    void updateChildOfParent(Node<T>* node, Node<T>* tempNode);
    void handleRecoloring(Node<T>* node, Node<T>* uncle, Node<T>* grandparent);
    Node<T>* insert(Node<T>* node, Node<T>* nodeToInsert);


        public:
    RedBlackTree();

    Node<T>* searchTree(T k);
    void leftRotate(Node<T>* x);
    void rightRotate(Node<T>* x);
    void insert(T key);
    void printTree();
};

// Fix linking error for templates
#include "tree.cpp"
#endif