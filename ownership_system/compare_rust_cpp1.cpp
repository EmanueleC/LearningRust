// compare rust and c++
#include <iostream>
#include <vector>

int main(){
  std::vector<int> data;
  data.push_back(1);
  data.push_back(2);
  int* x = &data[0];
  data.push_back(3); // reallocation: pointer x is invalidated
  std::cout << *x;
}
