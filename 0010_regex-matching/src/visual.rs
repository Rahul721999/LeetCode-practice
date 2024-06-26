
/// first row initialization ------
// if i == 0 && j=='*'{
//       dp[i][j] = dp[i][j-2]; 
// }

/// skip for the first row---
// if i == 0 {continue}

/// matching same char------
// if s[i] == p[j] || p[j] == '.'{
//       dp[i][j] = dp[i-1][j-1];
// }
// or p.charAt(j) == '*'----------
// else if p[j] == '*'{ 
//       // looking for zero occurrence----
//        if dp[i][j-2] == true{dp[i][j] == dp[i][j-2]}
//       // looking for multiple occurrences-----
//        else if s[i] == p[j-1] || p[j-1] == '.'{
//                   dp[i][j] = dp[i-1][j]
//        }
// }


// 0 1 2 3 4 5 6 7 8 9 10 11
//   m i s s i s s i p p  i
//   m i s * i s * i p *  .


//             i->| 0   1   2   3   4   5   6   7   8   9   10
//             -------------------------------------------------
//       j        |"" | m | i | s | s | i | s | i | p | p | i |
//    -----------------------------------------------------------
//       0     "" | T | F | F | F | F | F | F | F | F | F | F |
//             --------------------------------------------------
//       1     m  | F | T | F | F | F | F | F | F | F | F | F |
//             --------------------------------------------------
//       3     i  | F | F | T | F | F | F | F | F | F | F | F |
//             --------------------------------------------------
//       4     s  | F | F | F | T | F | F | F | F | F | F | F |
//             --------------------------------------------------
//       5     *  | F | F | T | T | T |   |   |   |   |   |   |
//             --------------------------------------------------
//       6     i  | F |   |   |   |   |   |   |   |   |   |   |
//             --------------------------------------------------
//       7     s  | F |   |   |   |   |   |   |   |   |   |   |
//             --------------------------------------------------
//       8     *  | F |   |   |   |   |   |   |   |   |   |   |
//             --------------------------------------------------
//       9     p  | F |   |   |   |   |   |   |   |   |   |   |
//             --------------------------------------------------
//       10    *  | F |   |   |   |   |   |   |   |   |   |   |
//             --------------------------------------------------
//       11    .  | F |   |   |   |   |   |   |   |   |   |   |
