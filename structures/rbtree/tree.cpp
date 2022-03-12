// Implementing Red-Black Tree

#include <iostream>
#include <string>
#include "tree.hpp"

template<typename T>
RedBlackTree<T>::RedBlackTree() {
    // Initially make all the elements in the tree empty
    root = nullptr;
};


/*
 * Return the null tree leaf
 */
template<typename T>
Node<T>& RedBlackTree<T>::getNull() {
    return *TNULL;
}

template<typename T>
void RedBlackTree<T>::insert(T key) {
    // If there are no nodes in the tree yet, insert the key into the root
    Node<T>* to_insert = new Node<T>(key, red);

    if (root == nullptr) {
        to_insert->setColor(black);
        root = to_insert;
        return;
    }

    // The tree has nodes in it.
    // Start by finding the appropriate leaf nodes
    Node<T>* current = root;
    while (true){
        if (current->getData() > key){
            if (!current->hasRight()){
                current->setRight(to_insert);
                to_insert->setParent(current);
                break;
            }
            current = current->getRight();
        }else{
            if (!current->hasLeft()){
                current->setLeft(to_insert);
                to_insert->setParent(current);
                break;
            }
            current = current->getLeft();
        }
    }
    // Start checking for imbalances

    // If the inserted nodes parent if black
    std::cout << "Inserting " << to_insert->getData() << "." << std::endl;
    insertFix(to_insert);
    std::cout << "Fixing " << to_insert->getData() << "." << std::endl;
};

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
        if (to_return->getData() == k){
            return to_return;
        }
        to_return = (to_return->getData() > k) ? to_return->getRight() : to_return->getLeft();
    }
     return to_return;
}

template<typename T>
void RedBlackTree<T>::leftRotate(Node<T>* x){
    Node<T>* y = x->getRight();
    x->setRight(y->getLeft());

    if (!y->hasLeft()){
        y->getLeft()->setParent(x);
    }
    y->setParent(x->getParent());

    if (!x->hasParent()){
        root = y;
    }else if (x == x->getParent()->getLeft()){
        x->getParent()->setLeft(y);
    }else{
        x->getParent()->setRight(y);
    }

    y->setLeft(x);
    x->setParent(y);
}

template<typename T>
void RedBlackTree<T>::rightRotate(Node<T>* x) {
    Node<T>* y = x->getLeft();
    x->setLeft(y->getRight());

    if (!y->hasRight()){
        y->getRight()->setParent(x);
    }
    y->setParent(x->getParent());

    if (!x->hasParent()){
        root = y;
    }else if (x == x->getParent()->getRight()){
        x->getParent()->setRight(y);
    }else{
        x->getParent()->setLeft(y);
    }

    y->setRight(x);
    x->setParent(y);
}

template<typename T>
void RedBlackTree<T>::insertFix(Node<T>* k){
    if (!k->hasParent() || !k->getParent()->hasParent()){return;}

    while(k->getParent()->getColor() == red){
        if (k->getParent() == k->getParent()->getParent()->getLeft()){
            Node<T>* y = k->getParent()->getParent()->getRight();
            if (y->getColor() == red) {
                k->getParent()->setColor(black);
                y->setColor(black);
                k->getParent()->getParent()->setColor(red);
                k = k->getParent()->getParent();
            }else{
                if (k == k->getParent()->getRight()){
                    k->setParent(k);
                    leftRotate(k);
                }
                k->getParent()->setColor(black);
                k->getParent()->getParent()->setColor(red);
                rightRotate(k);
            }
        }else{
            Node<T>* y = k->getParent()->getParent()->getLeft();
            if (y->getColor() == red) {
                k->getParent()->setColor(black);
                y->setColor(black);
                k->getParent()->getParent()->setColor(red);
                k = k->getParent()->getParent();
            }else{
                if (k == k->getParent()->getLeft()){
                    k->setParent(k);
                    rightRotate(k);
                }
                k->getParent()->setColor(black);
                k->getParent()->getParent()->setColor(red);
                leftRotate(k);
            }
        }
    }
    root->setColor(black);
}

template<typename T>
void RedBlackTree<T>::printTree() {
    if (root != nullptr) {
        printHelper(root, "", true);
    }
}

template<typename T>
void RedBlackTree<T>::printHelper(Node<T>* root, std::string indent, bool last) {
    if (root != nullptr) {
        std::cout << indent;
        if (last) {
            std::cout << "R----";
            indent += "   ";
        } else {
            std::cout << "L----";
            indent += "|  ";
        }

        std::string sColor = root->getColor() ? "RED" : "BLACK";
        std::cout << root->getData() << "(" << sColor << ")" << std::endl;
        printHelper(root->getLeft(), indent, false);
        printHelper(root->getRight(), indent, true);
    }
}
