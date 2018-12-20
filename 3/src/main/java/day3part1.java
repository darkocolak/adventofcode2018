import com.google.gson.Gson;
import com.google.gson.stream.JsonReader;

import java.io.*;
import java.net.URL;

public class day3part1 {
    public static void main(String[] args) throws FileNotFoundException {

        int[][] canvas = new int[1000][1000];
        int inchesWithMoreThanOneClaim = 0;
        AreaClaim[] claims = parseInput();
        for (AreaClaim claim : claims) {
            claim.setupDimensions();
            for (int i = claim.x; i < claim.x + claim.width; i++) {
                for (int j = claim.y; j < claim.y + claim.height; j++) {
                    canvas[i][j]++;
                }
            }
        }
        for (int i = 0; i < canvas.length; i++) {
            for (int j = 0; j < canvas[i].length; j++) {
                if (canvas[i][j] > 1) {
                    inchesWithMoreThanOneClaim++;
                }
            }
        }
        System.out.println("Result: " + inchesWithMoreThanOneClaim);
    }

    public static AreaClaim[] parseInput() throws FileNotFoundException {
        Gson gson = new Gson();
        URL resource = day3part1.class.getResource("/input.json");
        JsonReader reader = new JsonReader(new FileReader(resource.getFile()));
        return gson.fromJson(reader, AreaClaim[].class);
    }
    private class AreaClaim {
        int x, y, width, height;
        String surface;

        void setupDimensions(){
            String[] xes = this.surface.split("x");
            this.width = Integer.parseInt(xes[0]);
            this.height = Integer.parseInt(xes[1]);
        }
    }
}


