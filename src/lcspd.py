def LCSPD(s1: str, s2: str):
  mat = [[0 for x in range(len(s2)+1)] for x in range(len(s1)+1)]

  print_matrix("Initialized:", mat)

  for i in range(1, len(s1)+1):
    for j in range(1, len(s2)+1):
      if s1[i-1] == s2[j-1]:
        mat[i][j] = mat[i-1][j-1] + 1
      else:
        mat[i][j] = bigger(mat[i-1][j], mat[i][j-1])

  print_matrix("Final:", mat)

  return mat[i][j]

def bigger(v1, v2):
  if v1 >= v2:
    return v1 
  else:
    return v2

def print_matrix(label, matrix):
  print(label)
  print('\n'.join([' '.join([str(cell) for cell in row]) for row in matrix]))

args = [
  ["aggtab", "gxtxayb"],
  ["ANALISE", "ALGORITMOS"],
  ["PROGRAMACAODINAMICA", "TECNICADEOTIMIZACAO"],
  ["ABCDEFGHIJKLMNOPQRSTUVWXYZ", "ZYXWVUTSRQPONMLKJIHGFEDCBA"]
]

for [v1, v2] in args:
  print("{} - {}:".format(v1, v2), LCSPD(v1, v2))