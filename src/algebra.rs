/*
 * light is a path tracer written in Rust for educational purposes
 *
 * Copyright (C) 2024  Javier Lancha Vázquez
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub fn solve_deg2_eq(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a != 0.0 {
        let discriminant: f64 = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let x1 = (-b + sqrt_discriminant) / (2.0 * a);
            let x2 = (-b - sqrt_discriminant) / (2.0 * a);

            // Sort solutions
            if x1 <= x2 {
                return Some((x1, x2));
            } else {
                return Some((x2, x1));
            }
        } else if discriminant == 0.0 {
            let x = -b / (2.0 * a);
            return Some((x, x));
        } else {
            return None;
        }
    } else {
        if b != 0.0 {
            let x = -c / b;
            return Some((x, x));
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn deg2_eq_solutions() {
        let (a, b, c) = (-1.0, 2.0, 3.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 3.0)), solutions);

        let (a, b, c) = (1.0, 1.0, 0.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 0.0)), solutions);

        let (a, b, c) = (2.0, 2.0, 0.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 0.0)), solutions);

        let (a, b, c) = (0.0, 1.0, 2.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-2.0, -2.0)), solutions);

        let (a, b, c) = (0.0, 0.0, 2.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(None, solutions);

        let (a, b, c) = (1.0, 0.0, -1.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 1.0)), solutions);

        let (a, b, c) = (1.0, 2.0, 1.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, -1.0)), solutions);

        let (a, b, c) = (1.0, 2.0, 3.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(None, solutions);
    }
}
