#include <iostream>
#include <chrono>
#include <string>

using namespace std;

int main() {
    // Optimize I/O
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string greeting = "hello world";

    for(int i=0;i<1000;++i)
    {
        greeting+="abcd";
        cout<<(char*)greeting[0];
    }

    cout << greeting;

    char char1 = greeting[0];

    if (char1) {
        cout << char1;
    } else {
        cout << "No character";
    }

    // Start the timer
    auto start_time = chrono::high_resolution_clock::now();

    for (int i = 0; i < 100000; ++i) {
        cout << i << endl;
    }

    auto end_time = chrono::high_resolution_clock::now();
    auto elapsed_time = chrono::duration_cast<chrono::milliseconds>(end_time - start_time);

    cout << "Time taken: " << elapsed_time.count() << " ms" << endl;

    return 0;
}
