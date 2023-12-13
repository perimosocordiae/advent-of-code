! Usage: gfortran day11.f90 && ./a.out && rm -f a.out
program main
   implicit none
   character(len=*), parameter :: input = 'inputs/11.full'
   print *, 'Part 1:',solve_part(input, 1)
   print *, 'Part 2:', solve_part(input, 999999)
contains

   function solve_part(filename, incr) result(sum_dist)
      character(len=*), intent(in) :: filename
      integer, intent(in) :: incr
      integer(8) :: sum_dist
      integer :: num_stars, idx, last_star, gap
      integer, allocatable :: xs(:), ys(:)

      call parse_text(filename, xs, ys, num_stars)

      ! Increment ys(i:) by incr for each empty line above i.
      last_star = ys(1)
      do idx = 2, num_stars
         gap = ys(idx) - last_star - 1
         if (gap > 0) then
            ys(idx:) = ys(idx:) + incr * gap
         end if
         last_star = ys(idx)
      end do

      ! Sort the stars by xs.
      call bubble_sort(xs, ys)

      ! Increment xs(i:) by incr for each empty column to the left of i.
      last_star = xs(1)
      do idx = 2, num_stars
         gap = xs(idx) - last_star - 1
         if (gap > 0) then
            xs(idx:) = xs(idx:) + incr * gap
         end if
         last_star = xs(idx)
      end do

      ! Compute the sum of cityblock distances between each pair of stars.
      sum_dist = sum_distances(xs, ys)
   end function solve_part

   subroutine bubble_sort(arr1, arr2)
      integer, intent(inout) :: arr1(:), arr2(:)
      integer :: i,j,last,temp1,temp2

      last=size(arr1)
      do i=last-1,1,-1
         do j=1,i
            if (arr1(j+1).lt.arr1(j)) then
               temp1=arr1(j+1)
               temp2=arr2(j+1)
               arr1(j+1)=arr1(j)
               arr2(j+1)=arr2(j)
               arr1(j)=temp1
               arr2(j)=temp2
            endif
         enddo
      enddo
   end subroutine bubble_sort

   function sum_distances(xs, ys) result(sum_dist)
      integer, intent(in) :: xs(:), ys(:)
      integer(8) :: sum_dist, dx, dy
      integer :: idx, jdx

      sum_dist = 0
      do idx = 1, size(xs)
         do jdx = idx + 1, size(xs)
            dx = xs(idx) - xs(jdx)
            dy = ys(idx) - ys(jdx)
            sum_dist = sum_dist + abs(dx) + abs(dy)
         end do
      end do
   end function sum_distances

   subroutine parse_text(filename, xs, ys, num_stars)
      character(len=*), intent(in) :: filename
      integer, allocatable, intent(out) :: xs(:), ys(:)
      integer, intent(out) :: num_stars
      integer :: file_unit, rc, len_text
      integer :: num_cols, num_rows
      integer :: idx, pos
      character(len=:), allocatable :: text

      ! Read the file once to get the file size.
      open(action='read', file=filename, iostat=rc, newunit=file_unit)
      if (rc /= 0) then
         print *, 'Error opening file'
         stop
      end if
      inquire(unit=file_unit, size=len_text)
      close(file_unit)

      ! Allocate the character array and fill it.
      allocate(character(len=len_text) :: text)
      open(action='read', file=filename, iostat=rc, newunit=file_unit, &
         access='stream', form='unformatted')
      read(file_unit) text
      close(file_unit)

      ! Determine the number of rows and columns.
      num_cols = index(text, char(10)) - 1
      num_rows = len_text / (num_cols + 1)

      ! Count the number of stars ('#' characters) in the text.
      num_stars = 0
      idx = 0
      do
         pos = index(text(idx+1:), '#')
         if (pos == 0) then
            exit
         end if
         num_stars = num_stars + 1
         idx = idx + pos
      end do

      ! Fill the coordinates of the stars.
      allocate(xs(1:num_stars), ys(1:num_stars))
      num_stars = 0
      idx = 0
      do
         pos = index(text(idx+1:), '#')
         if (pos == 0) then
            exit
         end if
         num_stars = num_stars + 1
         idx = idx + pos
         xs(num_stars) = mod(idx, num_cols + 1)
         ys(num_stars) = idx / (num_cols + 1) + 1
      end do
   end subroutine parse_text

end program main
