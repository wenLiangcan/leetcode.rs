//Design a HashMap without using any built-in hash table libraries.
//
// Implement the MyHashMap class:
//
//
// MyHashMap() initializes the object with an empty map.
// void put(int key, int value) inserts a (key, value) pair into the HashMap.
//If the key already exists in the map, update the corresponding value.
// int get(int key) returns the value to which the specified key is mapped, or -
//1 if this map contains no mapping for the key.
// void remove(key) removes the key and its corresponding value if the map
//contains the mapping for the key.
//
//
//
// Example 1:
//
//
//Input
//["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
//[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
//Output
//[null, null, null, 1, -1, null, 1, null, -1]
//
//Explanation
//MyHashMap myHashMap = new MyHashMap();
//myHashMap.put(1, 1); // The map is now [[1,1]]
//myHashMap.put(2, 2); // The map is now [[1,1], [2,2]]
//myHashMap.get(1);    // return 1, The map is now [[1,1], [2,2]]
//myHashMap.get(3);    // return -1 (i.e., not found), The map is now [[1,1], [2
//,2]]
//myHashMap.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the
//existing value)
//myHashMap.get(2);    // return 1, The map is now [[1,1], [2,1]]
//myHashMap.remove(2); // remove the mapping for 2, The map is now [[1,1]]
//myHashMap.get(2);    // return -1 (i.e., not found), The map is now [[1,1]]
//
//
//
// Constraints:
//
//
// 0 <= key, value <= 10⁶
// At most 10⁴ calls will be made to put, get, and remove.
//
// Related Topics Array Hash Table Linked List Design Hash Function 👍 2113 👎 2
//35

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut m = MyHashMap::new();
        m.put(1, 1);
        m.put(2, 2);
        assert_eq!(m.get(1), 1);
        assert_eq!(m.get(3), -1);
        m.put(2, 1);
        m.put(2, 3);
        assert_eq!(m.get(2), 3);
        m.remove(2);
        assert_eq!(m.get(2), -1);
    }

    #[test]
    fn test_2() {
        let ops = vec!["MyHashMap", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "get", "put", "get", "put", "remove", "get", "put", "put", "put", "put", "get", "remove", "get", "remove", "get", "put", "put", "put", "put", "put", "put", "remove", "remove", "put", "put", "remove", "get", "put", "put", "put", "remove", "put", "get", "put", "get", "put", "remove", "put", "put", "get", "remove", "put", "put", "remove", "put", "put", "put", "put", "put", "remove", "put", "remove", "put", "put", "put", "put", "remove", "put", "get", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "remove", "put", "put", "put", "get", "put", "put", "put", "remove", "remove", "put", "put", "remove", "remove", "put", "put", "put", "remove", "remove", "put", "put", "put", "put", "put", "put", "remove", "get", "put", "get", "get", "put", "put", "remove", "put", "get", "put", "get", "put", "put", "put", "put", "put", "remove", "put", "remove", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "remove", "remove", "remove", "put", "put", "put", "put", "put", "get", "put", "put", "put", "get", "get", "remove", "put", "put", "put", "put", "remove", "get", "get", "put", "put", "get", "remove", "get", "put", "put", "put", "remove", "put", "get", "put", "get", "put", "put", "put", "remove", "put", "remove", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "remove", "remove", "remove", "put", "put", "put", "get", "put", "put", "put", "put", "remove", "get", "put", "get", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "remove", "remove", "remove", "put", "remove", "get", "put", "get", "put", "get", "put", "put", "put", "put", "put", "get", "remove", "put", "put", "put", "put", "remove", "put", "put", "get", "put", "put", "put", "remove", "put", "put", "put", "put", "remove", "put", "remove", "remove", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "get", "put", "get", "put", "get", "get", "put", "put", "remove", "put", "remove", "put", "put", "put", "put", "remove", "get", "get", "remove", "put", "put", "put", "remove", "get", "get", "put", "put", "put", "put", "put", "remove", "get", "put", "put", "put", "get", "put", "put", "remove", "put", "put", "put", "get", "get", "put", "put", "put", "remove", "put", "get", "put", "put", "remove", "put", "remove", "remove", "put", "put", "put", "put", "get", "put", "put", "put", "remove", "remove", "put", "get", "put", "put", "put", "put", "put", "remove", "put", "put", "put", "get", "remove", "put", "remove", "put", "put", "put", "remove", "put", "put", "put", "remove", "put", "remove", "put", "get", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "remove", "remove", "put", "put", "put", "put", "put", "put", "put", "remove", "get", "put", "put", "put", "put", "put", "put", "put", "remove", "put", "put", "put", "get", "get", "get", "put", "get", "put", "put", "get", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "get", "put", "put", "put", "put", "remove", "put", "put", "remove", "put", "put", "remove", "get", "put", "put", "get", "get", "get", "put", "put", "put", "get", "remove", "put", "get", "remove", "put", "get", "put", "get", "get", "put", "remove", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "put", "get", "get", "put", "put", "put", "get", "put", "put", "remove", "remove", "put", "put", "put", "remove", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "remove", "put", "put", "put", "put", "get", "get", "put", "put", "remove", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "put", "put", "put", "put", "put", "get", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "put", "get", "put", "put", "get", "put", "get", "put", "put", "put", "put", "get", "remove", "put", "remove", "get", "remove", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "put", "remove", "put", "remove", "put", "put", "put", "put", "get", "remove", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "get", "put", "put", "remove", "put", "remove", "get", "put", "put", "put", "get", "get", "remove", "put", "put", "remove", "put", "put", "put", "put", "put", "get", "put", "put", "remove", "get", "get", "put", "remove", "put", "put", "get", "get", "put", "get", "put", "remove", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "put", "get", "put", "put", "put", "get", "get", "remove", "put", "put", "remove", "get", "put", "get", "remove", "put", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "put", "put", "remove", "get", "get", "remove", "put", "remove", "put", "put", "get", "put", "put", "put", "put", "get", "put", "get", "put", "remove", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "put", "put", "get", "get", "put", "remove", "remove", "put", "remove", "get", "get", "put", "put", "put", "remove", "remove", "get", "get", "remove", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "get", "remove", "put", "put", "put", "put", "get", "put", "remove", "put", "get", "get", "get", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "get", "put", "put", "put", "put", "put", "put", "get", "put", "get", "put", "put", "put", "remove", "get", "get", "put", "put", "remove", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "remove", "put", "put", "remove", "put", "put", "remove", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "remove", "put", "put", "remove", "get", "get", "put", "put", "get", "put", "get", "get", "put", "put", "put", "put", "get", "remove", "put", "put", "put", "put", "get", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "get", "put", "remove", "put", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "remove", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "put", "put", "put", "put", "remove", "put", "put", "put", "put", "put", "remove", "remove", "put", "put", "remove", "put", "put", "put", "put", "put", "get", "remove", "get", "put", "get", "remove", "put", "get", "remove", "get", "put", "put", "put", "put", "put", "put", "put", "put", "remove", "remove", "get", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put", "remove", "get", "remove", "remove", "put", "put", "get", "put", "put", "put", "put", "put", "remove", "put", "put", "remove", "put", "put", "put", "put", "put", "put", "remove", "remove", "remove", "put", "put", "put", "get", "put", "put", "put", "remove", "put", "put"];
        let data = vec![vec![], vec![970, 538], vec![908, 29], vec![395, 865], vec![250, 847], vec![836], vec![233, 568], vec![657, 790], vec![595, 271], vec![769, 219], vec![55, 112], vec![157, 493], vec![920], vec![632, 358], vec![669], vec![506, 228], vec![904], vec![473], vec![461, 40], vec![748, 973], vec![446, 544], vec![766, 461], vec![395], vec![211], vec![415], vec![157], vec![252], vec![252, 22], vec![942, 681], vec![600, 988], vec![424, 39], vec![685, 482], vec![561, 605], vec![632], vec![461], vec![916, 329], vec![739, 735], vec![769], vec![942], vec![460, 226], vec![183, 411], vec![224, 524], vec![769], vec![508, 614], vec![632], vec![254, 825], vec![603], vec![115, 667], vec![460], vec![940, 813], vec![50, 629], vec![519], vec![595], vec![39, 913], vec![742, 13], vec![765], vec![163, 627], vec![554, 130], vec![255, 945], vec![22, 78], vec![912, 390], vec![632], vec![609, 410], vec![956], vec![515, 243], vec![975, 871], vec![520, 313], vec![682, 538], vec![563], vec![119, 902], vec![916], vec![766], vec![293, 885], vec![657, 665], vec![518, 832], vec![897, 489], vec![340, 972], vec![790, 24], vec![637, 445], vec![544, 498], vec![115], vec![600], vec![269, 209], vec![734, 3], vec![243, 108], vec![233], vec![679, 632], vec![640, 55], vec![288, 301], vec![682], vec![871], vec![922, 755], vec![624, 787], vec![776], vec![293], vec![564, 902], vec![32, 743], vec![934, 278], vec![250], vec![389], vec![620, 711], vec![420, 623], vec![346, 959], vec![829, 832], vec![776, 894], vec![465, 769], vec![508], vec![358], vec![126, 481], vec![255], vec![50], vec![477, 991], vec![973, 337], vec![32], vec![823, 283], vec![21], vec![733, 431], vec![583], vec![735, 407], vec![873, 702], vec![578, 256], vec![813, 221], vec![669, 432], vec![790], vec![941, 945], vec![645], vec![560, 775], vec![823, 772], vec![458, 220], vec![243], vec![947, 136], vec![168, 560], vec![946, 19], vec![172, 608], vec![624, 260], vec![876, 516], vec![76, 334], vec![704], vec![615, 737], vec![110, 453], vec![189, 678], vec![746, 201], vec![497, 330], vec![632], vec![993], vec![497], vec![915, 545], vec![329, 558], vec![662, 773], vec![208, 135], vec![797, 614], vec![640], vec![108, 809], vec![262, 401], vec![560, 965], vec![494], vec![222], vec![922], vec![237, 439], vec![688, 796], vec![974, 331], vec![367, 470], vec![339], vec![115], vec![790], vec![658, 873], vec![513, 520], vec![2], vec![76], vec![110], vec![199, 511], vec![674, 853], vec![546, 849], vec![430], vec![394, 41], vec![189], vec![443, 983], vec![270], vec![353, 44], vec![959, 995], vec![275, 798], vec![910], vec![522, 939], vec![176], vec![520, 292], vec![631, 538], vec![216, 551], vec![197, 623], vec![55], vec![452, 416], vec![805, 462], vec![884, 223], vec![484, 454], vec![126], vec![59, 954], vec![454, 575], vec![500, 940], vec![999, 520], vec![211, 106], vec![190, 915], vec![995, 462], vec![519], vec![368], vec![734], vec![549, 845], vec![47, 885], vec![609, 356], vec![765], vec![163, 520], vec![382, 774], vec![190, 545], vec![146, 166], vec![748], vec![282], vec![357, 531], vec![113], vec![89, 489], vec![994, 5], vec![967], vec![972, 826], vec![220, 91], vec![296, 808], vec![826, 7], vec![401, 910], vec![126, 802], vec![534, 781], vec![293], vec![282], vec![515], vec![29, 473], vec![390], vec![32], vec![265, 801], vec![658], vec![29, 648], vec![678], vec![40, 493], vec![9, 809], vec![701, 244], vec![345, 542], vec![386, 296], vec![632], vec![89], vec![575, 758], vec![168, 575], vec![286, 619], vec![210, 335], vec![546], vec![77, 534], vec![380, 885], vec![414], vec![590, 272], vec![924, 933], vec![662, 1], vec![544], vec![349, 402], vec![244, 274], vec![907, 303], vec![638, 453], vec![339], vec![255, 802], vec![973], vec![130], vec![266, 695], vec![486, 444], vec![557, 522], vec![518], vec![392, 750], vec![538, 174], vec![407, 337], vec![455, 912], vec![755, 278], vec![242], vec![947, 647], vec![367], vec![657, 280], vec![286], vec![255], vec![307, 453], vec![382, 216], vec![609], vec![69, 582], vec![340], vec![36, 792], vec![4, 288], vec![518, 878], vec![376, 516], vec![873], vec![211], vec![417], vec![908], vec![695, 90], vec![646, 513], vec![114, 675], vec![924], vec![321], vec![486], vec![300, 87], vec![68, 223], vec![924, 665], vec![680, 323], vec![410, 97], vec![538], vec![685], vec![884, 617], vec![496, 967], vec![155, 853], vec![367], vec![694, 765], vec![607, 567], vec![748], vec![565, 855], vec![151, 633], vec![300, 838], vec![76], vec![163], vec![796, 591], vec![516, 514], vec![95, 921], vec![353], vec![445, 40], vec![32], vec![163, 846], vec![697, 892], vec![637], vec![870, 672], vec![560], vec![410], vec![909, 414], vec![319, 973], vec![259, 144], vec![423, 142], vec![165], vec![984, 652], vec![121, 133], vec![117, 13], vec![340], vec![390], vec![237, 305], vec![520], vec![712, 752], vec![380, 7], vec![661, 937], vec![249, 9], vec![937, 291], vec![168], vec![501, 89], vec![989, 540], vec![736, 722], vec![739], vec![250], vec![741, 270], vec![920], vec![421, 848], vec![917, 716], vec![788, 294], vec![520], vec![642, 352], vec![250, 775], vec![710, 268], vec![349], vec![801, 427], vec![921], vec![905, 183], vec![69], vec![163, 429], vec![274, 863], vec![108], vec![410, 289], vec![578, 258], vec![613, 113], vec![992, 527], vec![155, 194], vec![557, 696], vec![190, 519], vec![630, 59], vec![234, 795], vec![740, 324], vec![270, 492], vec![107, 63], vec![705, 446], vec![205], vec![390, 676], vec![374], vec![557], vec![321, 754], vec![710], vec![121], vec![488], vec![156, 679], vec![132, 238], vec![793, 469], vec![966, 912], vec![612, 691], vec![860, 749], vec![245, 736], vec![829], vec![237], vec![858, 449], vec![655, 591], vec![913, 760], vec![977, 737], vec![13, 954], vec![189, 252], vec![215, 933], vec![168], vec![682, 867], vec![802, 747], vec![710, 342], vec![994], vec![257], vec![288], vec![972, 139], vec![255], vec![637, 821], vec![383, 858], vec![685], vec![146, 53], vec![464, 179], vec![98, 642], vec![911], vec![524, 259], vec![196, 641], vec![644, 843], vec![271, 723], vec![706, 285], vec![205], vec![256, 884], vec![291, 722], vec![750, 142], vec![205, 50], vec![259], vec![816, 839], vec![123, 741], vec![913, 349], vec![894, 577], vec![914], vec![129, 267], vec![242, 60], vec![288], vec![189, 30], vec![507, 805], vec![277], vec![227], vec![332, 850], vec![716, 309], vec![493], vec![401], vec![669], vec![682, 330], vec![880, 925], vec![4, 287], vec![858], vec![177], vec![351, 217], vec![621], vec![193], vec![611, 128], vec![183], vec![789, 817], vec![655], vec![658], vec![501, 965], vec![412], vec![334, 651], vec![345, 411], vec![73, 9], vec![162, 791], vec![644], vec![330, 608], vec![998, 312], vec![96, 234], vec![906, 419], vec![221], vec![179, 324], vec![507, 978], vec![632], vec![134], vec![155, 691], vec![92, 442], vec![353], vec![861], vec![714, 403], vec![824, 261], vec![814, 164], vec![118], vec![931, 816], vec![39, 844], vec![790], vec![894], vec![980, 797], vec![673, 80], vec![109, 321], vec![293], vec![256], vec![625, 277], vec![272, 650], vec![890, 666], vec![282, 399], vec![50, 95], vec![635, 169], vec![25, 749], vec![681, 759], vec![126, 84], vec![229, 309], vec![892, 941], vec![505, 343], vec![496], vec![843, 820], vec![259, 362], vec![549, 86], vec![340], vec![602, 374], vec![711, 73], vec![196], vec![519, 269], vec![785, 465], vec![747, 706], vec![449, 878], vec![407], vec![357], vec![459, 357], vec![615, 903], vec![191], vec![86, 672], vec![467, 638], vec![406, 351], vec![665, 29], vec![416, 514], vec![739], vec![563, 810], vec![190], vec![402], vec![705, 575], vec![91, 324], vec![993, 183], vec![902, 528], vec![136, 560], vec![866, 309], vec![458], vec![13, 751], vec![999], vec![201, 651], vec![666, 246], vec![969, 696], vec![249, 255], vec![632, 413], vec![109, 432], vec![866, 745], vec![203, 476], vec![72, 869], vec![379, 178], vec![955, 758], vec![367], vec![864, 228], vec![13], vec![101, 120], vec![679, 497], vec![814], vec![216, 235], vec![401], vec![588, 717], vec![914, 836], vec![648, 29], vec![253, 926], vec![316], vec![69], vec![978, 205], vec![963], vec![701], vec![941], vec![758, 489], vec![506, 806], vec![597, 467], vec![618, 861], vec![863, 454], vec![190], vec![490, 703], vec![866, 151], vec![735, 303], vec![217, 552], vec![460], vec![850, 691], vec![554, 450], vec![1, 997], vec![424], vec![603, 68], vec![319], vec![883, 877], vec![489, 109], vec![819, 400], vec![957, 400], vec![467], vec![601], vec![481], vec![832, 895], vec![979, 52], vec![137, 90], vec![394, 540], vec![198, 303], vec![562, 223], vec![680, 851], vec![911, 399], vec![730, 427], vec![563], vec![983, 9], vec![559, 525], vec![422, 86], vec![121, 720], vec![531], vec![151, 902], vec![468, 501], vec![39], vec![932, 29], vec![650], vec![598], vec![131, 715], vec![752, 133], vec![119, 831], vec![789], vec![606], vec![941], vec![735, 652], vec![107, 448], vec![98], vec![436, 737], vec![728, 947], vec![533, 835], vec![971, 356], vec![776, 984], vec![880], vec![939, 191], vec![119, 844], vec![168], vec![733], vec![39], vec![739, 760], vec![940], vec![265, 768], vec![523, 176], vec![969], vec![269], vec![464, 948], vec![456], vec![183, 358], vec![559], vec![211, 986], vec![696, 242], vec![427, 90], vec![785], vec![868, 968], vec![976, 307], vec![667, 619], vec![798, 831], vec![297, 468], vec![802, 53], vec![221, 956], vec![319, 908], vec![791, 313], vec![56, 158], vec![494, 398], vec![387, 909], vec![298], vec![180, 573], vec![839], vec![663, 316], vec![460, 628], vec![674, 243], vec![254], vec![922], vec![197], vec![814, 245], vec![560, 195], vec![392], vec![788], vec![439, 556], vec![698], vec![746], vec![138, 331], vec![379, 700], vec![984, 296], vec![937, 471], vec![968, 888], vec![220, 87], vec![706, 368], vec![430, 702], vec![466, 349], vec![441], vec![334, 642], vec![324, 670], vec![635], vec![406], vec![734], vec![611], vec![954, 130], vec![666], vec![601, 106], vec![951, 372], vec![382], vec![64, 691], vec![481, 581], vec![91, 807], vec![113, 529], vec![307], vec![129, 955], vec![383], vec![926, 412], vec![644], vec![86, 862], vec![879, 202], vec![661, 959], vec![849, 806], vec![583, 501], vec![331, 825], vec![922], vec![391], vec![68, 504], vec![149, 731], vec![463, 64], vec![572, 623], vec![165, 760], vec![268], vec![448], vec![819, 282], vec![586], vec![45], vec![353, 316], vec![589], vec![655], vec![249], vec![382, 367], vec![963, 184], vec![625, 538], vec![932], vec![996], vec![935], vec![612], vec![788], vec![252, 397], vec![728, 935], vec![765, 924], vec![976], vec![784, 464], vec![806, 326], vec![625, 866], vec![512, 83], vec![569, 768], vec![473, 714], vec![401], vec![683], vec![603, 271], vec![762, 849], vec![449, 150], vec![242, 372], vec![285], vec![31, 474], vec![716], vec![653, 344], vec![673], vec![588], vec![813], vec![672, 111], vec![120, 299], vec![353, 497], vec![297, 510], vec![619, 677], vec![390], vec![50], vec![185, 215], vec![809, 322], vec![749, 218], vec![868, 920], vec![955, 207], vec![530, 64], vec![95, 906], vec![940, 961], vec![505, 893], vec![339, 428], vec![364, 122], vec![426], vec![646, 775], vec![455, 542], vec![423], vec![807], vec![142, 73], vec![392], vec![174, 439], vec![575, 427], vec![321, 697], vec![9, 998], vec![32, 272], vec![351, 496], vec![834], vec![481, 71], vec![760], vec![737, 574], vec![812, 451], vec![626, 196], vec![102], vec![648], vec![617], vec![166, 152], vec![701, 720], vec![750], vec![625, 425], vec![335, 404], vec![820, 328], vec![240], vec![226, 815], vec![508, 265], vec![785], vec![863, 881], vec![298, 263], vec![325], vec![12, 960], vec![439, 28], vec![502, 331], vec![114], vec![117, 930], vec![922, 65], vec![456], vec![686, 211], vec![668, 478], vec![601], vec![839, 336], vec![737, 282], vec![756, 191], vec![581, 547], vec![89], vec![167], vec![695], vec![272, 244], vec![449, 570], vec![688, 413], vec![142, 769], vec![382], vec![522], vec![794, 632], vec![714, 983], vec![163], vec![737], vec![310], vec![771, 745], vec![254, 395], vec![551], vec![765, 622], vec![263], vec![751], vec![986, 924], vec![76, 585], vec![548, 618], vec![589, 427], vec![785], vec![866], vec![541, 789], vec![58, 900], vec![816, 495], vec![324, 395], vec![626], vec![648, 807], vec![280, 557], vec![92], vec![994, 295], vec![852, 148], vec![700, 192], vec![626, 551], vec![628, 937], vec![442, 892], vec![268, 263], vec![536], vec![593, 528], vec![567], vec![164, 733], vec![997, 800], vec![583, 941], vec![967, 894], vec![737], vec![825, 103], vec![528, 774], vec![485, 373], vec![611], vec![935, 118], vec![190, 262], vec![494, 836], vec![917], vec![997, 448], vec![269, 659], vec![941, 894], vec![913, 554], vec![63, 246], vec![167], vec![235, 456], vec![136, 501], vec![309], vec![212, 503], vec![796, 931], vec![337], vec![129, 759], vec![302, 276], vec![360, 779], vec![809, 918], vec![55], vec![888, 433], vec![535, 160], vec![97], vec![769, 627], vec![762], vec![449, 596], vec![703, 27], vec![133, 407], vec![509, 67], vec![266], vec![697, 778], vec![903, 748], vec![430, 909], vec![464, 626], vec![452, 493], vec![891], vec![293], vec![20, 945], vec![686, 88], vec![329], vec![207, 827], vec![202, 641], vec![709, 186], vec![303, 451], vec![385, 541], vec![409], vec![201], vec![766], vec![395, 956], vec![971], vec![704], vec![250, 34], vec![354], vec![766], vec![789], vec![106, 974], vec![652, 301], vec![654, 875], vec![109, 363], vec![481, 935], vec![361, 686], vec![989, 931], vec![488, 673], vec![979], vec![806], vec![450], vec![509, 414], vec![528, 835], vec![673], vec![328, 121], vec![303, 406], vec![45, 174], vec![410, 906], vec![42, 356], vec![675, 953], vec![882, 222], vec![978], vec![252, 396], vec![347, 94], vec![367, 754], vec![578, 278], vec![107, 628], vec![73, 583], vec![166, 100], vec![162], vec![593], vec![791], vec![740], vec![697, 977], vec![368, 152], vec![971], vec![918, 197], vec![152, 444], vec![995, 814], vec![869, 900], vec![335, 818], vec![250], vec![951, 948], vec![605, 293], vec![959], vec![873, 243], vec![236, 239], vec![577, 636], vec![244, 441], vec![884, 904], vec![942, 993], vec![708], vec![581], vec![82], vec![293, 859], vec![622, 486], vec![878, 920], vec![220], vec![493, 182], vec![445, 582], vec![392, 227], vec![523], vec![800, 270], vec![404, 803]];
        let outputs = vec![-2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -1, -2, -2, -1, -2, -2, -2, -2, 865, -2, -1, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 681, -2, -2, -2, -2, -2, -1, -2, -1, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 329, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 568, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, 945, 629, -2, -2, -2, -2, -1, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 108, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 55, -2, -2, -2, -1, -1, -2, -2, -2, -2, -2, -2, -1, -1, -2, -2, -1, -2, 453, -2, -2, -2, -2, -2, 678, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -1, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, 873, -2, -1, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, 470, -2, 619, 802, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 106, -1, -2, -2, -2, -2, -2, -1, 444, -2, -2, -2, -2, -2, -2, 482, -2, -2, -2, 470, -2, -2, -2, -2, -2, -2, -1, 520, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, 292, -2, -2, -2, -2, -2, -2, -2, -2, -2, 735, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 582, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -1, 696, -2, 268, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 305, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 5, -1, 301, -2, 802, -2, -2, 482, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 144, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -1, 910, 432, -2, -2, -2, 449, -2, -2, -1, -2, -2, 411, -2, 591, 873, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -1, -1, -2, -2, -1, -1, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 967, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, 337, 531, -2, -2, -2, -2, -2, -2, -2, -2, 735, -2, 519, -1, -2, -2, -2, -2, -2, -2, 220, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 751, -2, -2, 164, -2, 910, -2, -2, -2, -2, -1, -2, -2, -2, 244, -2, -2, -2, -2, -2, -2, 519, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 638, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -1, -2, -2, -2, 817, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, 925, -2, -2, -2, 431, -1, -2, -2, -2, -2, 696, 209, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, 825, -1, -2, -2, -2, -2, 294, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 351, -1, -2, -2, -2, -2, -2, 216, -2, -2, -2, -2, 453, -2, 858, -2, -2, -2, -2, -2, -2, -2, -2, -1, -1, -2, -2, -2, -2, -2, -1, -1, -2, -2, -2, -2, -2, 591, 255, -2, -2, -2, -2, -2, -1, 691, -2, -2, -2, -2, 307, -2, -2, -2, -2, -2, -2, 910, -2, -2, -2, -2, -2, -1, -2, -2, -2, 80, 717, 221, -2, -2, -2, -2, -2, 676, 95, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, 142, -1, -2, -1, -2, -2, -2, -2, -2, -2, -1, -2, -1, -2, -2, -2, -2, 29, -1, -2, -2, -2, -2, -2, -2, -1, -2, -2, -1, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -1, 90, -2, -2, -2, -2, 367, -2, -2, -2, -2, 282, -1, -2, -2, -1, -2, -1, -1, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, 196, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, 282, -2, -2, -2, -1, -2, -2, -2, 716, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -1, -2, -2, -1, -2, 849, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -1, -2, 356, -2, -2, -1, -2, 817, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -1, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 205, -2, -2, -2, -2, -2, -2, -2, -2, 528, -2, -2, -2, -2, 356, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 87, -2, -2, -2, -2, -2, -2];
        let ops = ops.into_iter().zip(data.iter()).zip(outputs.iter());
        let mut m = MyHashMap::new();
        for ((op, d), o) in ops.skip(1) {
            match op {
                "put" => m.put(d[0], d[1]),
                "remove" => m.remove(d[0]),
                "get" => assert_eq!(m.get(d[0]), *o, "getting {}", d[0]),
                _ => ()
            }
        }
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
struct Entry {
    key: i32,
    value: i32,
    next: Option<Box<Entry>>
}

impl Entry {
    fn new(key: i32, value: i32) -> Self {
        Entry {
            key,
            value,
            next: None
        }
    }
}

struct MyHashMap {
    buckets: Vec<Option<Box<Entry>>>,
    size: usize
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 *
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
impl MyHashMap {

    fn init_buckets(size: usize) -> Vec<Option<Box<Entry>>> {
        let mut buckets = vec![];
        for _ in 0 .. size {
            buckets.push(None);
        }
        buckets
    }

    fn new() -> Self {
        MyHashMap {
            buckets: Self::init_buckets(16),
            size: 0
        }
    }

    fn _hash(capacity: usize, key: i32) -> usize {
        key.abs() as usize % capacity
    }

    fn hash(&self, key: i32) -> usize {
        Self::_hash(self.buckets.len(), key)
    }

    fn rehash(&mut self) {
        if self.size as f32 >= self.buckets.len() as f32 * 0.75 {
            let mut buckets = Self::init_buckets(self.buckets.len() * 2);
            for mut n in &self.buckets {
                while let Some(e) = n {
                    Self::put_entry(&mut buckets, e.key, e.value);
                    n = &e.next;
                }
            }
            self.buckets = buckets;
        }
    }

    fn put_entry(buckets: &mut Vec<Option<Box<Entry>>>, key: i32, value: i32) -> bool {
        let idx = Self::_hash(buckets.len(), key);
        let bucket = buckets[idx].as_mut();
        if let Some(mut e) = bucket {
            loop {
                if (*e).key == key {
                    (*e).value = value;
                    return false;
                } else if (*e).next.is_some() {
                    let next = (*e).next.as_mut().unwrap();
                    e = next;
                } else {
                    (*e).next = Some(Box::new(Entry::new(key, value)));
                    return true;
                }
            }
        } else {
            buckets[idx] = Some(Box::new(Entry::new(key, value)));
            return true;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if Self::put_entry(&mut self.buckets, key, value) {
            self.size += 1;
            self.rehash();
        }
    }

    fn get(&self, key: i32) -> i32 {
        let idx = self.hash(key);
        let mut bucket = self.buckets[idx].as_ref();
        while let Some(e) = bucket {
            if (*e).key == key {
                return (*e).value;
            } else {
                bucket = (*e).next.as_ref();
            }
        }
        return -1;
    }

    fn remove(&mut self, key: i32) {
        let idx = self.hash(key);
        let bucket = self.buckets[idx].as_mut();
        if let Some(mut e) = bucket {
            if (*e).key == key {
                let next = (*e).next.take();
                self.buckets[idx] = next;
                self.size -= 1;
            } else {
                while (*e).next.is_some() {
                    let next = (*e).next.as_ref().unwrap();
                    if (*next).key == key {
                        let nn = (*e.next.as_mut().unwrap()).next.take();
                        if let Some(nn) = nn {
                            (*e).next.replace(nn);
                        } else {
                            (*e).next.take();
                        }
                        self.size -= 1;
                        return;
                    } else {
                        e = (*e).next.as_mut().unwrap();
                    }
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
