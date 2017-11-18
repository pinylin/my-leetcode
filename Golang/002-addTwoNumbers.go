/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
// 64.83  ai!
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {

	var head *ListNode
	var rear *ListNode
	carry := 0

	for l1 != nil || l2 != nil || carry != 0 {
		//sum := l1.Val + l2.Val + carry
		sum := 0
		if nil != l1 {
			sum += l1.Val
			l1 = l1.Next
		}
		if nil != l2 {
			sum += l2.Val
			l2 = l2.Next
		}
		sum += carry
		carry = sum / 10
		node := &ListNode{
			sum % 10,
			nil,
		}
		// 指针初始
		if nil == head {
			head = node
			rear = node
		} else {
			rear.Next = node
			rear = node
		}

	}
	return head
}
