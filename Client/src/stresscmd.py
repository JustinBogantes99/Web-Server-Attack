import subprocess
import sys
'''
recibe por argumentos el numero de hilos que va a trabajar y el executable 
'''
number_threads = int(sys.argv[1])
binary = sys.argv[2]
i = 0
'''
el siguiente fragmento ejecuta el codigo con la cantidad de hilos '''
while i != number_threads:
    subprocess.run([binary, sys.argv[3], sys.argv[4],sys.argv[5]])
    i += 1