---
layout: archive
title: C++ sort 함수 사용법
tags: [C++]
category: "Programming"
---

C++ algorithm 라이브러리의 sort 함수에 대한 설명과 그 사용법에 대하여 정리한다.

# 1. Sort 함수

C++의 sort 함수는 `array`나 `vector`와 같이 `Random Access Iterator`를 갖는 컨테이너에 대하여 정렬을 해주는 함수이다.

> `list`는 `Random Access Iterator`를 갖지 않기 때문에 sort 함수를 사용할 수 없다.

sort 함수는 `<algotirhm>` 헤더에 정의되어 있으며, 다음과 같은 형상을 가지고 있다.

```cpp
template <class RandomAccessIterator>  void sort (RandomAccessIterator first, RandomAccessIterator last);
template <class RandomAccessIterator, class Compare>  void sort (RandomAccessIterator first, RandomAccessIterator last, Compare comp);
```

`first`부터 `last` 범위 내에 있는 원소들을 비교하여 정렬하며, `operator<`를 이용하여 원소들 간의 크기를 정렬하거나, `comp` 인자가 있는 경우, `comp`로 크기 비교를 수행한다.

> **first, last**
Random-access iterators to the initial and final positions of the sequence to be sorted. The range used is [first,last), which contains all the elements between first and last, including the element pointed by first but not the element pointed by last.
RandomAccessIterator shall point to a type for which swap is properly defined and which is both move-constructible and move-assignable.

> **comp**
Binary function that accepts two elements in the range as arguments, and returns a value convertible to bool. The value returned indicates whether the element passed as first argument is considered to go before the second in the specific strict weak ordering it defines.
The function shall not modify any of its arguments.
This can either be a function pointer or a function object.

# 2. 사용 예

## 기본 예

기본적으로 사용되는 `operator<`를 사용하여 정렬(오름차순)

```cpp
#include <algorithm>

int main(void)
{
  // 배열 정렬
  int a[5] = {3, 2, 0, 4, 1};
  std::sort(a, a + 5);
  
  // 벡터 정렬
  std::vector<int> v = {3, 2, 0, 4, 1};
  std::sort(v.begin(), v.end());
  
  return 0;
}
```

## comp를 이용한 예

sort 함수 인자에 compare 함수를 이용하여 해당 조건에 맞게 정렬하며, 해당 예시는 내림차순으로 정렬된다.

```cpp
#include <algorithm>

bool compare(int a, int b){
  return a > b; // a가 b보다 크면 true
}
int main(void)
{
  // 배열 정렬
  int a[5] = {3, 2, 0, 4, 1};
  std::sort(a, a + 5, compare);
  
  // 벡터 정렬
  std::vector<int> v = {3, 2, 0, 4, 1};
  std::sort(v.begin(), v.end(), compare);
  
  return 0;
}
```

## class의 operator< 오버로딩을 이용한 예

sort 함수는 기본적으로 `operator<`를 이용하여 정렬하므로, 연산자 오버로딩을 통해 해당 클래스를 정렬시킬 수 있다.

```cpp
#include <algorithm>

class Student{
public:
  Student(string name, int age) : name_(name), age_(age){}
  
  bool operator<(Student s) const {
    if(this->name == s.name){return this->age < s.age;}
    else {return this->name < s.name;}
  }
private:
  std::string name_;
  int age_;
};

int main(void)
{
  std::vector<Student> v;
  
  v.push_back(Student("a", 30));
  v.push_back(Student("b", 22));
  v.push_back(Student("c", 41));
  v.push_back(Student("d", 17));
  
  std::sort(v.begin(), v.end());
  
  return 0;
}
```

# 참고문헌

- [std::sort - algorithm - CPlusPlus.com](https://cplusplus.com/reference/algorithm/sort/)
