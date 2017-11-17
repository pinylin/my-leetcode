#1 (Brute Force) [Accepted]
/*
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        vector<int> res;
        for(int i=0; i<nums.size(); i++){
            for(int j=i+1; j<nums.size(); j++){
                if(nums[i] + nums[j] == target){
                    res.push_back(i);
                    res.push_back(j);
                    return res;
                }
            }
        }

    }
};
*/

#2 Hash
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> hash;
        vector<int> result;
        for (int i = 0; i < nums.size(); i++) {
            int numberToFind = target - nums[i];

            if (hash.find(numberToFind) != hash.end()) {
                result.push_back(hash[numberToFind]);
                result.push_back(i);
                return result;
            }

            hash[nums[i]] = i;
        }
        return result;

    }
};
