# Smart Pointers in C++

## Why Smart Pointers?

When working with raw pointers in C or C++, it’s easy to introduce subtle and dangerous memory errors
especially *use-after-free*, *double free*, and *memory leaks*.

Consider this C example:

```c
int main(int argc, char **argv) {
    char line[128];
    while (1) {
        printf("auth = %p, service = %p\n", auth, service);
        if (fgets(line, sizeof(line), stdin) == NULL) break;

        if (strncmp(line, "auth", 4) == 0) {
            auth = malloc(sizeof(auth));
            memset(auth, 0, sizeof(auth));
            if (strlen(line + 5) < 31) {
                strcpy(auth->name, line + 5);
            }
        }
        if (strncmp(line, "reset", 5) == 0) {
            free(auth);  // (#1) pointer is freed here
        }
        if (strncmp(line, "login", 5) == 0) {
            if (auth->auth) {  // (#2) use-after-free (auth might be freed!)
                printf("You have logged in already.\n");
            } else {
                printf("Re-enter your password again.\n");
            }
        }
    }
}
````

Even the syntax is correct, this code can crash because it uses a pointer (`auth`) **after freeing it**.
These types of bugs are extremely common and hard to detect.

C++ provides **Smart Pointers** to make pointer management safer and automatic.

---

## Example Problem with Raw Pointers

Let’s look at a basic example in C++:

```c
#include <iostream>
#include <cstring>

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

void do_something_with_the_dog(Dog *d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
}

int main() {
    Dog *ralf = new Dog();
    do_something_with_the_dog(ralf);
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
```

This works fine — until we modify the function to delete the dog:

```c
void do_something_with_the_dog(Dog *d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
    delete d;  // (#1) deleting the dog
}

int main() {
    Dog *ralf = new Dog();
    do_something_with_the_dog(ralf);  // (#2) the dog is deleted here
    ralf->setName("Lica");            // (#3) use-after-free -> crash
    ralf->speak();
    return 0;
}
```

After deleting `ralf` inside the function, `main()` still tries to access it — causing a **crash**.

---

## Enter Smart Pointers

C++ provides **smart pointers** in `<memory>` to automatically manage object lifetimes.

Let’s replace the raw pointer with a **`std::unique_ptr`**:

```c
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

void do_something_with_the_dog(Dog *d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
}

int main() {
    std::unique_ptr<Dog> ralf = std::make_unique<Dog>();
    do_something_with_the_dog(ralf.get());
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
```

Here, `std::unique_ptr<Dog>` owns the object.
When `ralf` goes out of scope, the `Dog` instance is **automatically deleted** — no leaks, no manual `delete`.

---

## Ownership and `std::move`

`unique_ptr` represents **exclusive ownership** — only one smart pointer can own an object.
If you try to copy it, you’ll get a compile-time error.

To **transfer ownership**, you must use `std::move()`:

```c 
void do_something_with_the_dog(std::unique_ptr<Dog> d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
}

int main() {
    std::unique_ptr<Dog> ralf = std::make_unique<Dog>();
    do_something_with_the_dog(std::move(ralf));  // ownership moved
    ralf->setName("Lica");  // crash: ralf no longer owns the Dog
    return 0;
}
```

After `std::move`, `ralf` becomes empty (null).
To safely continue using the object, the function must **return** ownership:

```c
std::unique_ptr<Dog> do_something_with_the_dog(std::unique_ptr<Dog> d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
    return d;  // return ownership
}

int main() {
    std::unique_ptr<Dog> ralf = std::make_unique<Dog>();
    ralf = do_something_with_the_dog(std::move(ralf));
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
```

This works correctly — ownership is transferred back to `main()`.

---

## Using `.get()` to Pass Raw Pointers

Sometimes, you only need to **use** the object temporarily without taking ownership.
You can call `.get()` on a smart pointer to retrieve the raw pointer it manages:

```c
void do_something_with_the_dog(const Dog *d) {
    d->speak();
}

int main() {
    std::unique_ptr<Dog> ralf = std::make_unique<Dog>();
    do_something_with_the_dog(ralf.get());  // no ownership transfer
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
```

To ensure safety, we mark the pointer parameter as `const` so the function can’t modify or delete it.

---

## Shared Ownership with `std::shared_ptr`

If you need **multiple owners** of the same object, use `std::shared_ptr`.

```c
#include <memory>

void do_something_with_the_dog(std::shared_ptr<Dog> d) {
    d->setName("Hi, I am the pet dog Lica. You can rename me!");
    d->speak();
}

int main() {
    std::shared_ptr<Dog> ralf = std::make_shared<Dog>();
    do_something_with_the_dog(ralf);  // increases reference count
    ralf->setName("Lica");
    ralf->speak();
    return 0;
}
```

Each `shared_ptr` keeps a **reference counter**.
When all `shared_ptr`s pointing to the object go out of scope, the object is automatically deleted.

---

## Summary

| Smart Pointer Type | Ownership  | Copyable | Common Use Case                          |
| ------------------ | ---------- | -------- | ---------------------------------------- |
| `std::unique_ptr`  | Exclusive  | ❌ No     | Sole ownership, RAII resource management |
| `std::shared_ptr`  | Shared     | ✅ Yes    | Shared ownership between components      |
| `std::weak_ptr`    | Non-owning | ✅ Yes    | Break cyclic references in shared graphs |

---

### Key Takeaways

* **Use `unique_ptr`** when a single function or class owns a resource.
* **Use `shared_ptr`** when ownership is shared.
* **Avoid raw pointers** unless interfacing with legacy code or external libraries.
* Let **RAII (Resource Acquisition Is Initialization)** handle cleanup automatically.

---

**In short:**

> Smart pointers give you the power of pointers — without the pain of manual memory management.

