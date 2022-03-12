#ifndef TREE_H 
#define TREE_H

#include <cstdlib>
#include <string>
#include "node.hpp"

template<typename T>
class RedBlackTree{
  private:
    Node<T> *root;
    Node<T> *TNULL;
//    void initializeNULLNode(Node<T>& node, Node<T>&  parent);
//    void preOrderHelper(Node<T>& node);
//    void inOrderHelper(Node<T>& node);
//    void postOrderHelper(Node<T>& node);
//    Node<T>& searchTreeHelper(Node<T>& node, int key);
//    void deleteFix(Node<T>& x);
//    void rbTransplant(Node<T>& u, Node<T>& v);
//    void deleteNodeHelper(Node<T>& node, int key);
    void insertFix(Node<T>* k);
    void printHelper(Node<T>* root, std::string indent, bool last);


  public:
    RedBlackTree();
//    void preorder();
//    void inorder();
//    void postorder();
    Node<T>& getNull();
    Node<T>* searchTree(T k);
//    Node<T>& minimum(Node<T>& node);
//    Node<T>& maximum(Node<T>& node);
//    Node<T>& successor(Node<T>& x);
//    Node<T>& predecessor(Node<T>& x);
    void leftRotate(Node<T>* x);
    void rightRotate(Node<T>* x);
    void insert(T key);
//    Node<T>* getRoot();
//    void deleteNode(T data);
    void printTree();
};

// Fix linking error for templates
#include "tree.cpp"
#endif