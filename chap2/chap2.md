2.1

a. `[ac]*[bc]*`

b. `([bc]*(a[bc]*a)*)*`

c. `[10]+00`

d. `1[01]{6,}|11[01]{4}|1011[01]{2}|10101[10]`

e. `[ac]*b*a?[bc]*`

f. `0[0-7]*|[1-9][0-9]*`

g. `1|10`


<!-- This is the original graph
digraph G {
   start[label= "", shape=none,height=.0,width=.0];
   {node[shape=circle];1;2;4;6;8};
   {node[shape=doublecircle];7;9}
   
   start->1
   1->2[label="a"]
   2->6[label="b"]
   2->4[label="ε"]
   4->4[label="a"]
   4->6[label="c"]
   6->7[label="x"]
   1->8[label="ε"]
   8->8[label="x"]
   8->9[label="a"]
   2->7[label="ε"]
   7->2[label="ε"]
}
-->
![Alt text](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%3B%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B4%3B6%3B8%7D%3B%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B7%3B9%7D%0A%20%20%20%0A%20%20%20start-%3E1%0A%20%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E6%5Blabel%3D%22b%22%5D%0A%20%20%202-%3E4%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%204-%3E4%5Blabel%3D%22a%22%5D%0A%20%20%204-%3E6%5Blabel%3D%22c%22%5D%0A%20%20%206-%3E7%5Blabel%3D%22x%22%5D%0A%20%20%201-%3E8%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%208-%3E8%5Blabel%3D%22x%22%5D%0A%20%20%208-%3E9%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E7%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%207-%3E2%5Blabel%3D%22%CE%B5%22%5D%0A%7D)
