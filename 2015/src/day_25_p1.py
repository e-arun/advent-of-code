from libs import io

input = io.read_all().split()
row = int(input[-3].rstrip(","))
col = int(input[-1].rstrip("."))


diag_idx = row + col - 1
diag_max_num = (diag_idx * (diag_idx + 1)) // 2
diag_cur_num = diag_max_num - row + 1

ans = 20151125 * pow(252533, diag_cur_num - 1, 33554393)
ans = ans % 33554393
print(ans)
