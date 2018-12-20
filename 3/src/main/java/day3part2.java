import com.google.gson.Gson;
import com.google.gson.stream.JsonReader;

import java.io.FileNotFoundException;
import java.io.FileReader;
import java.net.URL;

public class day3part2 {
    public static void main(String[] args) throws FileNotFoundException {
        int[][] canvas = new int[1000][1000];
        AreaClaim[] claims = parseInput();
        for (AreaClaim claim : claims) {
            claim.setupDimensions();
            for (int i = claim.x; i < claim.x + claim.width; i++) {
                for (int j = claim.y; j < claim.y + claim.height; j++) {
                    canvas[i][j]++;
                }
            }
        }
        for (AreaClaim claim : claims) {
            if(claim.checkIfNoOverlaps(canvas)){
                System.out.println("ID: " + claim.id);
            }
        }
    }

    static AreaClaim[] parseInput() throws FileNotFoundException {
        Gson gson = new Gson();
        URL resource = day3part2.class.getResource("/input.json");
        JsonReader reader = new JsonReader(new FileReader(resource.getFile()));
        return gson.fromJson(reader, AreaClaim[].class);
    }

    private class AreaClaim {
        int id, x, y, width, height;
        String surface;

        void setupDimensions(){
            String[] xes = this.surface.split("x");
            this.width = Integer.parseInt(xes[0]);
            this.height = Integer.parseInt(xes[1]);
        }

        boolean checkIfNoOverlaps(int[][] canvas){
            for (int i = x; i < x + width; i++) {
                for (int j = y; j < y + height; j++) {
                    if (canvas[i][j] != 1) {
                        return false;
                    }
                }
            }
            return true;
        }

    }
}


