import requests
import sys

filter_variants = False
token = ''

if __name__ == '__main__':
    if (len(sys.argv) > 1):
        filter_variants = (sys.argv[1] != 'false')

    if (len(sys.argv) > 2):
        token = sys.argv[2]


    print('Filter-variants: %s token: %s' % (filter_variants, token != ''))
