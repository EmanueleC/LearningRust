// compare rust and c++
#include <iostream>
#include <vector>
using namespace std;

int main(){
  vector<string> data;
  data.push_back("uno");
  data.push_back("due");
  string* x = &data[0];
  data.push_back("tre"); // reallocation: pointer x is invalidated
  cout << *x;
}
