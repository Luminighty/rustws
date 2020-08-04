import sys

def mode_char(line):
	for c in line:
		c = c.capitalize()
		if c == 'S':
			out.write(' ')
		elif c == 'T':
			out.write('\t')
		elif c == 'L':
			out.write('\n')
		
is_mode_char = True
out = open(('.'.join(sys.argv[1].split('.')[0:-1]) + ".ws"), "w")
for line in open(sys.argv[1]):
	if not line.startswith('--'):
		if is_mode_char:
			mode_char(line)

