use decafc::compile;

fn main() {
    // TODO: example code
let code = r#"  
// a quicksort program.  set the "length" parameter in main() to the
// desired size of the sorted array.  if you want to sort an array
// bigger than 100 elements, you'll also need to adjust the declaration
// of the global array A.

class Program
{
    int A[100];
    int length;
    
    int partition(int p, int r) 
    {
	int x, i, j, t;
        int z;
        
	x = A[p];
	i = p - 1;
	j = r + 1;

  	for z = 0, length * length {
	  j = j - 1;
	    for a = 0, length {
	      if (A[j] <= x) {
		break;
	      }
	      j = j - 1;
	    }

	    for a = i + 1, length {
	      if (A[a] >= x) {
		i = a;
		break;
	      }
	    }

   	    if (i < j) {
  		t = A[i];
  		A[i] = A[j];
  		A[j] = t;
  	    } else {
 		return j;
  	    }
  	}
	return -1;
    }

    void quicksort(int p, int r)
    {
  	int q;
        
  	if (p < r) {
  	    q = partition (p, r);
  	    quicksort (p, q);
  	    quicksort (q+1, r);
  	}
    }
    
    void main() 
    {
	int temp;
        
	length = 10; // adjust for sort length
        
        callout("printf", "creating random array of %d elements\n", length);

        callout("srandom", 17);
        
	for i = 0, length {
            temp = callout("random");
            A[i] = temp;
        }
        
        callout("printf", "\nbefore sort:\n");
	for i = 0, length {
   	    callout ("printf", "%d\n", A[i]); 
        }
        
        quicksort (0, length - 1);

        callout("printf", "\nafter sort\n");
	for i = 0, length {
	  callout ("printf", "%d\n", A[i]); 
  	}
    }
}
"#;    
    let code = String::from(code);
    compile(&code);
}
