#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <deque>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// uncomment to disable assert()
#define NDEBUG

using namespace std;

// The pathmap points towards the previous node in the optimal path.
using PathMap = unordered_map<string, string>;
using CostMap = unordered_map<string, int32_t>;
using LengthMap = unordered_map<int, vector<string>>;

// Utilities for debugging.
std::ostream& operator<<(std::ostream& os, const PathMap& m) {
  os << "PathMap:\n";
  for (const auto& pair : m) {
    os << "Key: " << pair.first << "\nValue: ";
    os << pair.second << "\n";
  }
  return os;
}

std::ostream& operator<<(std::ostream& os, const CostMap& m) {
  os << "CostMap:\n";
  for (const auto& pair : m) {
    os << "Key: " << pair.first << "\nValue: " << pair.second << "\n";
  }
  return os;
}

bool is_single_edit(const string& word1, const string& word2) {
  bool first_word_longer = word1.size() > word2.size();
  // The iterators into the longer word.
  auto long_it = (first_word_longer) ? word1.begin() : word2.begin();
  auto long_end = (first_word_longer) ? word1.end() : word2.end();
  // The iterators into the shorter word.
  auto short_it = (!first_word_longer) ? word1.begin() : word2.begin();

  bool found_edit = false;
  while (long_it != long_end) {
    if (*long_it != *short_it) {
      if (found_edit) {
        return false;
      } else {
        ++long_it;
        found_edit = true;
        continue;
      }
    }
    ++long_it;
    ++short_it;
  }

  return true;
}

vector<string> find_string_mutations(
  const LengthMap& words_by_length,
  const string& input_word) {
  vector<string> output;
  auto it = words_by_length.find(input_word.size()-1);
  if (it != words_by_length.end()) {
    for (const auto& word : it->second) {
      if (is_single_edit(input_word, word)) {
        output.push_back(word);
      }
    }
  }

  it = words_by_length.find(input_word.size()+1);
  if (it != words_by_length.end()) {
    for (const auto& word : it->second) {
      if (is_single_edit(input_word, word)) {
        output.push_back(word);
      }
    }
  }

  return output;
}

void initialize_maps(
  const vector<string>& words,
  LengthMap& words_by_length,
  PathMap& paths, CostMap& cost_map) {
  for (const auto& word : words) {
    cost_map[word] = INT32_MAX;
    words_by_length[word.size()].push_back(word);
  }
}

void add_words(vector<string> new_words,
               deque<string>& words_deque) {
  words_deque.insert(words_deque.end(), make_move_iterator(new_words.begin()),
                     make_move_iterator(new_words.end()));
}

PathMap find_paths(const vector<string>& words, const string& source) {
  PathMap path_map;
  CostMap cost_map;
  LengthMap words_by_length;
  initialize_maps(words, words_by_length, path_map, cost_map);

  cost_map[source] = 0;
  deque<string> words_to_visit = {source};
  add_words(find_string_mutations(words_by_length, source), words_to_visit);

  unordered_set<string> visited_words;
  while (words_to_visit.size()) {
    const string current_word = std::move(*words_to_visit.begin());
    words_to_visit.pop_front();
    // Move on if we've handled this word before.
    if (visited_words.find(current_word) != visited_words.end()) continue;
    visited_words.insert(current_word);

    assert(cost_map[current_word] < INT32_MAX);
    int32_t new_cost = cost_map[current_word] + 1;

    auto connected_words = find_string_mutations(words_by_length, current_word);
    add_words(connected_words, words_to_visit);

    for (const auto& word : connected_words) {
      auto current_cost = cost_map[word];
      if (new_cost < current_cost) {
        cost_map[word] = new_cost;
        path_map[word] = std::move(current_word);
      }
    }
  }

  return path_map;
}

void run_asserts() {
  {
    const vector<string> words = {"bare", "bar", "bart", "fart"};
    PathMap expected = {{words[1], words[0]}, {words[2], words[1]}};
    assert(find_paths(words, "bare") == expected);
  }
  {
    const vector<string> words = {"bare", "bar", "bart", "fart"};
    PathMap expected = {{words[0], words[1]}, {words[2], words[1]}};
    assert(find_paths(words, "bar") == expected);
  }
}

int main() {
  run_asserts();

  cout << "Enter the dictionary filename: \n";
  string filename;
  cin >> filename;

  vector<string> words;

  ifstream words_file(filename);
  if (words_file.is_open()) {
    string line;
    while (getline(words_file, line)) {
      if (line.size() <= 2) continue;
      words.push_back(std::move(line));
    }
  } else {
    cout << "Failed to open file\n";
    return 1;
  }
  cout << "Enter the source word: \n";
  string source;
   cin >> source;

  // If the source isn't in our list of words, give up.
  if (find(words.begin(), words.end(), source) == words.end()) {
    cout << "Source word not found!\n";
    return 1;
  }

  PathMap path_map = find_paths(words, source);

  cout << "Enter the target word: \n";
  string target;
  cin >> target;
  if (find(words.begin(), words.end(), target) == words.end()) {
    cout << "Target not found!\n";
    return 1;
  }

  const string* path_step = &path_map[target];
  if (path_step == nullptr) {
    cout << "No path to target!\n";
    return 1;
  }

  vector<string> shortest_path = {*path_step};
  while (*path_step != source) {
    path_step = &path_map[*path_step];
    assert(!path_step->empty());
    shortest_path.push_back(*path_step);
  }

  cout << "Found path: \n";
  for (auto short_path_it = shortest_path.rbegin();
       short_path_it != shortest_path.rend(); ++short_path_it) {
    cout << *short_path_it << " -> ";
  }
  cout << target << "\n";

  return 0;
}
