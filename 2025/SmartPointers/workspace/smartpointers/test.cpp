#include <iostream>
#include <memory>

class Dog {
public:
    void speak();
    void setName(std::string input);

private:
    std::string name;
};

void Dog::setName(std::string input) {
    this->name = input;
}

void Dog::speak() {
    std::cout << "Woof woof! I am " << this->name << std::endl;
}

//void do_something_with_the_dog(Dog *d) {
void do_something_with_the_dog(std::shared_ptr<Dog> d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
}

int main() {
    //std::unique_ptr<Dog> ralf = std::make_unique<Dog>();
    std::shared_ptr<Dog> ralf = std::make_shared<Dog>();

    do_something_with_the_dog(ralf);
    // do_something_with_the_dog(ralf.get());
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
