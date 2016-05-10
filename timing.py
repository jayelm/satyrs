"""
Measure timing of satyrs on cnf files of varying sizes.
"""
from subprocess import check_output, STDOUT
from numpy import std

# In (roughly) increasing order
CNF_FILES = [
		'./tests/uf250-01.cnf'
		#'./tests/test.cnf',
	#'./tests/quinn.cnf',
	# './tests/medium.cnf',
	# './tests/cascade.cnf',
    # Unsat
	# './tests/phole/hole6.cnf',
	# './tests/phole/hole7.cnf'
	#   './tests/dubois29_unsat.cnf'
]

if __name__ == '__main__':
    from argparse import ArgumentParser, ArgumentDefaultsHelpFormatter
    parser = ArgumentParser(formatter_class=ArgumentDefaultsHelpFormatter)
    parser.add_argument('-n', '--num', type=int, default=5,
                        help="Number of times to run satyrs on each cnf")
    parser.add_argument('-v', '--verbose', action='store_true',
                        help="Be verbose (print the results of each run)")
    args = parser.parse_args()

    for f in CNF_FILES:
        average_time = 0
        runs = []
        for i in xrange(args.num):
            output = check_output(
                ["time","target/debug/satyrs", f],
                stderr=STDOUT,
            )
            # Output from time command is the last six elements of this
            # split, and real time is what we're looking for
			#print output
            timing_output = output.split()[-6:]
            real_time = float(timing_output[0])
            if args.verbose:
                print "{} ({}): {}".format(f, i, real_time)
            average_time += real_time / args.num
            runs.append(real_time)
        print "{} (AVERAGE): {}\n\t(min: {} max: {} sd: {})".format(
            f, average_time, round(min(runs), 2),
            round(max(runs), 2), round(std(runs), 2)
        )
	
