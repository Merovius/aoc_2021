package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"math"
	"os"
	"strings"
)

func main() {
	read(os.Stdin)
}

func read(r io.Reader) {
	buf, err := io.ReadAll(r)
	if err != nil {
		panic(err)
	}
	buf = bytes.TrimSpace(buf)
	p, err := ParsePacket(string(buf))
	if err != nil {
		panic(err)
	}
	fmt.Printf("Sum of versions: %d\n", p.VersionSum())
	fmt.Printf("Evaluated: %d\n", p.Eval())
}

type Reader struct {
	data []uint64
	o    int // how many bits are left to be read in the first uint64
}

func ParsePacket(s string) (Packet, error) {
	b, err := hex.DecodeString(s)
	if err != nil {
		return nil, err
	}
	var data []uint64
	for len(b) > 8 {
		data = append(data, binary.BigEndian.Uint64(b))
		b = b[8:]
	}
	var buf [8]byte
	copy(buf[:], b)
	data = append(data, binary.BigEndian.Uint64(buf[:]))
	r := &Reader{data: data, o: 64}
	return r.ReadPacket()
}

func (r *Reader) ReadBits(n int) uint64 {
	if n > 64 {
		panic("too many bits read at once")
	}
	if len(r.data) == 0 || (len(r.data) == 1 && n > r.o) {
		panic(io.ErrUnexpectedEOF)
	}
	if n <= r.o {
		v := r.data[0] & ((1 << r.o) - 1)
		v >>= r.o - n
		r.o -= n
		if r.o == 0 {
			r.data = r.data[1:]
			r.o = 64
		}
		return v
	}
	o := r.o
	v := r.ReadBits(o)
	v <<= n - o
	return v | r.ReadBits(n-o)
}

func (r *Reader) ReadPacket() (Packet, error) {
	p, _, err := r.readPacket()
	return p, err
}

func (r *Reader) readPacket() (Packet, uint64, error) {
	ver := r.ReadBits(3)
	id := r.ReadBits(3)

	switch id {
	case 4:
		p, m, err := r.readLiteralPacket(ver)
		return p, m + 6, err
	default:
		p, m, err := r.readOperatorPacket(ver, id)
		return p, m + 6, err
	}
}

func (r *Reader) readLiteralPacket(ver uint64) (Packet, uint64, error) {
	n := uint64(0)

	var v uint64
	for i := 0; i < 64/4; i++ {
		n += 5
		v <<= 4
		d := r.ReadBits(5)
		v |= d & 0xf
		if d>>4 == 0 {
			return Literal{ver, v}, n, nil
		}
	}
	return nil, 0, errors.New("literal packet too large")
}

func (r *Reader) readOperatorPacket(ver, id uint64) (Packet, uint64, error) {
	mode := r.ReadBits(1)
	n := uint64(1)
	var sub []Packet
	switch mode {
	case 0:
		m := r.ReadBits(15)
		n += 15
		n += m
		for i := uint64(0); i < m; {
			p, k, err := r.readPacket()
			if err != nil {
				return nil, 0, err
			}
			i += k
			sub = append(sub, p)
		}
	case 1:
		m := r.ReadBits(11)
		n += 11
		for i := uint64(0); i < m; i++ {
			p, k, err := r.readPacket()
			if err != nil {
				return nil, 0, err
			}
			sub = append(sub, p)
			n += k
		}
	default:
		panic("r.ReadBits returned more than 1 bit")
	}
	return Operator{Ver: ver, ID: id, Sub: sub}, n, nil
}

type Packet interface {
	String() string
	VersionSum() uint64
	Eval() uint64
}

type Literal struct {
	Ver uint64
	Val uint64
}

func (l Literal) String() string {
	return fmt.Sprintf("%d", l.Val)
}

func (l Literal) VersionSum() uint64 {
	return l.Ver
}

func (l Literal) Eval() uint64 {
	return l.Val
}

type Operator struct {
	Ver uint64
	ID  uint64
	Sub []Packet
}

func (o Operator) String() string {
	var pieces []string
	for _, s := range o.Sub {
		pieces = append(pieces, s.String())
	}
	switch o.ID {
	case 0:
		return fmt.Sprintf("sum(%s)", strings.Join(pieces, ", "))
	case 1:
		return fmt.Sprintf("prod(%s)", strings.Join(pieces, ", "))
	case 2:
		return fmt.Sprintf("min(%s)", strings.Join(pieces, ", "))
	case 5:
		return fmt.Sprintf("gt(%s)", strings.Join(pieces, ", "))
	case 6:
		return fmt.Sprintf("lt(%s)", strings.Join(pieces, ", "))
	case 7:
		return fmt.Sprintf("eq(%s)", strings.Join(pieces, ", "))
	default:
		return fmt.Sprintf("%d(%s)", o.ID, strings.Join(pieces, ", "))
	}
}

func (o Operator) VersionSum() uint64 {
	v := o.Ver
	for _, s := range o.Sub {
		v += s.VersionSum()
	}
	return v
}

func (o Operator) Eval() (total uint64) {
	switch o.ID {
	case 0:
		for _, s := range o.Sub {
			total += s.Eval()
		}
	case 1:
		total = 1
		for _, s := range o.Sub {
			total *= s.Eval()
		}
	case 2:
		total = math.MaxUint64
		for _, s := range o.Sub {
			if v := s.Eval(); v < total {
				total = v
			}
		}
	case 3:
		for _, s := range o.Sub {
			if v := s.Eval(); v > total {
				total = v
			}
		}
	case 5:
		a, b := o.Sub[0].Eval(), o.Sub[1].Eval()
		if a > b {
			return 1
		}
		return 0
	case 6:
		a, b := o.Sub[0].Eval(), o.Sub[1].Eval()
		if a < b {
			return 1
		}
		return 0
	case 7:
		a, b := o.Sub[0].Eval(), o.Sub[1].Eval()
		if a == b {
			return 1
		}
		return 0
	default:
		panic(fmt.Sprintf("invalid operator ID %d", o.ID))
	}
	return total
}
