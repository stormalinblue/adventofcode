def num_solutions(string, string_pos, cons, cons_pos, dp=None):
    if dp is None:
        dp = {}
    if (string_pos, cons_pos) in dp:
        return dp[string_pos, cons_pos]
    # print(string_pos, cons_pos, repr(string[string_pos:]), cons[cons_pos:])
    if cons_pos == len(cons):
        # print('all cons satisfied')
        if not any(c == '#' for c in string[string_pos:]):
            dp[string_pos, cons_pos] = 1
            return 1
        else:
            dp[string_pos, cons_pos] = 0
            return 0
    elif string_pos >= len(string):
        dp[string_pos, cons_pos] = 0
        return 0
    else:
        if string[string_pos] == '.':
           dp[string_pos, cons_pos] = num_solutions(string, string_pos + 1, cons, cons_pos, dp)
           return dp[string_pos, cons_pos]
        elif string[string_pos] == '#':
            pos = string_pos
            sec_len = 0
            while pos < len(string) and sec_len < cons[cons_pos] and (string[pos] == '#' or string[pos] == '?'):
                pos += 1
                sec_len += 1
            
            # print(string_pos, cons_pos, 'hash: sec_len', sec_len, 'pos', pos, len(string))
            
            if pos != len(string) and string[pos] == '#':
                dp[string_pos, cons_pos] = 0
                return 0
            
            if sec_len == cons[cons_pos]:
                dp[string_pos, cons_pos] = num_solutions(string, pos + 1, cons, cons_pos + 1, dp)
                return dp[string_pos, cons_pos]
            else:
                dp[string_pos, cons_pos] = 0
                return 0
        else:
            # print(string_pos, cons_pos, 'found ?')
            # print(string_pos, cons_pos, 'entering if point')
            if_point = num_solutions(string, string_pos + 1, cons, cons_pos, dp)
            # print(string_pos, cons_pos, 'returned from if point', if_point)

            # print(string_pos, cons_pos, 'entering if hash')
            pos = string_pos
            sec_len = 0
            while pos < len(string) and sec_len < cons[cons_pos] and (string[pos] == '#' or string[pos] == '?'):
                pos += 1
                sec_len += 1
            
            if (pos != len(string) and string[pos] == '#'):
                dp[string_pos, cons_pos] = if_point
                return if_point
            
            if sec_len == cons[cons_pos]:
                # print(string_pos, cons_pos, 'found match')
                dp[string_pos, cons_pos] = if_point + num_solutions(string, pos + 1, cons, cons_pos + 1, dp)
                return dp[string_pos, cons_pos]
            else:
                dp[string_pos, cons_pos] = if_point
                return if_point

with open('input12.txt') as f:
    problems = []
    for line in f.read().strip().split('\n'):
        # print(line.count('#'))
        prob, _sat = line.split()
        # print(_sat, _sat.split(','))
        sat = [int(i) for i in _sat.split(',')]
        problems.append(('?'.join([prob] * 5), sat * 5))
    # print(problems)

    ans = 0
    for prob, sat in problems:
        # print(prob, sat)
        # print(len(prob), len(sat), len(prob) * len(sat))
        ans += num_solutions(prob, 0, sat, 0)
        # print(ans)
    print(ans)
                
                
    