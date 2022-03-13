// Implementing Red-Black Tree

#include <iostream>
#include <string>
#include <stack>
#include "tree.hpp"

template<typename T>
RedBlackTree<T>::RedBlackTree() {
    root = nullptr;
};

template<typename T>
RedBlackTree<T>::~RedBlackTree() {
    if (root == nullptr){ return;}
    // Get an array to all the pointers in the tree
    std::vector<Node<T>*> pointers = getAllNodes();

    for (Node<T>* pointer: pointers){
        delete pointer;
    }
}

template<typename T>
std::vector<Node<T>*> RedBlackTree<T>::getAllNodes(){
    std::vector<Node<T>*> pointers;
    std::vector<Node<T>*> to_traverse;

    if (root == nullptr) {return pointers;}

    to_traverse.push_back(root);
    while(!to_traverse.empty()){
        Node<T>* current = to_traverse.back();
        to_traverse.pop_back();
        if (current)
        pointers.push_back(current);

        if (current->hasLeft()){
            to_traverse.push_back(current->getLeft());
        }
        if (current->hasRight()){
            to_traverse.push_back(current->getRight());
        }
    }

    return pointers;
}

template<typename T>
void RedBlackTree<T>::insert(T key) {
    Node<T>* node = new Node<T>(key);
    root = insert(root, node);
    recolorAndRotate(node);
};

template<typename T>
Node<T>* RedBlackTree<T>::insert(Node<T>* node, Node<T>* nodeToInsert){
    if (node == nullptr){
        return nodeToInsert;
    }
    if (nodeToInsert->getData() < node->getData()){
        node->setLeft(insert(node->getLeft(), nodeToInsert));
        node->getLeft()->setParent(node);
    }else if (nodeToInsert->getData() > node->getData()){
        node->setRight(insert(node->getRight(), nodeToInsert));
        node->getRight()->setParent(node);
    }
    return node;
}

/*
 * Search the tree. If the key does not exist, then return none
 */
template<typename T>
Node<T>* RedBlackTree<T>::searchTree(T k) {
    Node<T>* to_return = root;
    // Trivial Case
    if (to_return == nullptr || to_return->getData() == k){
        return root;
    }
    while(to_return != nullptr) {
        std::cout << to_return->getData() << std::endl;
        if (to_return->getData() == k){
            return to_return;
        }
        to_return = (to_return->getData() < k) ? to_return->getRight() : to_return->getLeft();
    }
     return to_return;
}

template<typename T>
void RedBlackTree<T>::rightRotate(Node<T>* node) {
    Node<T>* leftNode = node->getLeft();
    node->setLeft(leftNode->getRight());
    if (node->hasLeft()){
        node->getLeft()->setParent(node);
    }
    leftNode->setRight(node);
    leftNode->setParent(node->getParent());
    updateChildOfParent(node, leftNode);
    node->setParent(leftNode);
}

template<typename T>
void RedBlackTree<T>::leftRotate(Node<T>* node){
    Node<T>* rightNode = node->getRight();
    node->setRight(rightNode->getLeft());
    if (node->hasRight()){
        node->getRight()->setParent(node);
    }
    rightNode->setLeft(node);
    rightNode->setParent(node->getParent());
    updateChildOfParent(node, rightNode);
    node->setParent(rightNode);
};

template<typename T>
void RedBlackTree<T>::updateChildOfParent(Node<T>* node, Node<T>* tempNode){
    if (!node->hasParent()){
        root = tempNode;
    }else if (node->isLeftChild()){
        node->getParent()->setLeft(tempNode);
    } else{
        node->getParent()->setRight(tempNode);
    }
}
template<typename T>
void RedBlackTree<T>::recolorAndRotate(Node<T>* node){
    Node<T>* parent = node->getParent();

    if(node != root && parent->getColor() == red) {
        Node<T> *grandparent = parent->getParent();
        Node<T> *uncle = parent->isLeftChild() ? grandparent->getRight() : grandparent->getLeft();

        if (uncle != nullptr && uncle->getColor() == red) {
            handleRecoloring(parent, uncle, grandparent);
        } else if (parent->isLeftChild()) {
            handleLeftSituations(node, parent, grandparent);
        } else if (!parent->isLeftChild()) {
            handleRightSituations(node, parent, grandparent);
        }
    }
    root->setColor(black);
}
template<typename T>
void RedBlackTree<T>::handleRecoloring(Node<T>* parent, Node<T>* uncle, Node<T>* grandparent){
    uncle->flipColor();
    parent->flipColor();
    grandparent->flipColor();
    recolorAndRotate(grandparent);

}

template<typename T>
void RedBlackTree<T>::handleRightSituations(Node<T>* node, Node<T>* parent, Node<T>* grandparent){
    if (node->isLeftChild()){
        rightRotate(parent);
    }
    parent->flipColor(); // swap colors
    grandparent->flipColor();
    leftRotate(grandparent);
    recolorAndRotate(node->isLeftChild() ? grandparent : parent);
}

template<typename T>
void RedBlackTree<T>::handleLeftSituations(Node<T>* node, Node<T>* parent, Node<T>* grandparent){
    if (!node->isLeftChild()){
        leftRotate(parent);
    }
    parent->flipColor(); // swap colors
    grandparent->flipColor();
    rightRotate(grandparent);
    recolorAndRotate(node->isLeftChild() ? parent : grandparent);
}

template<typename T>
void RedBlackTree<T>::printTree() {
    if (root == nullptr){return;}
    printHelper(root, 1);
}

template<typename T>
void RedBlackTree<T>::printHelper(Node<T>* node, int depth) {

    std::cout << "Depth: " << depth << "| Data: " << node->getData() << "| Parent: ";
    if (node->hasParent()){
        std::cout << node->getParent()->getData();
    }else{
        std::cout << "Null";
    }
    std::cout << "| Color: " << ((node->getColor() == red) ? "red" : "black");
    std::cout << std::endl;

    if (node->hasLeft()){
        printHelper(node->getLeft(), depth+1);
    }

    if (node->hasRight()){
        printHelper(node->getRight(), depth+1);
    }
}




